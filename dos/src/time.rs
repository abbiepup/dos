use core::arch::asm;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;

static mut TICKS: u32 = 0;
static mut OFFSET: u64 = 0;

/// A measurement of a monotonically nondecreasing clock.
/// Opaque and useful only with [`Duration`].
///
/// [`Instant`]s are always guaranteed, barring [platform bugs], to be no less than any previously
/// measured [`Instant`] when created, and are often useful for tasks such as measuring
/// benchmarks or timing how long an operation takes.
///
/// Note, however, that [`Instant`]s are **not** guaranteed to be **steady**. In other
/// words, each tick of the underlying clock might not be the same length (e.g.
/// some seconds may be longer than others). An [`Instant`] may jump forwards or
/// experience time dilation (slow down or speed up), but it will never go
/// backwards.
///
/// [`Instant`]s are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an [`Instant`]. Instead, it only
/// allows measuring the [`Duration`] between two instants (or comparing two [`Instant`]s).
///
/// [platform bugs]: Instant#monotonicity
///
/// # Monotonicity
///
/// [`Instant`] measures time using the BIOS tick counter (int 0x1A), which increments roughly
/// every 55 ms. [`now`] automatically handles midnight wraps to ensure a monotonically nondecreasing value.
/// In practice such guarantees are – under rare circumstances – broken by hardware, virtualization
/// or operating system bugs. To work around these bugs and platforms not offering monotonic clocks
/// [`duration_since`], [`elapsed`] and [`sub`] saturate to zero.
/// [`checked_duration_since`] can be used to detect and handle situations
/// where monotonicity is violated, or [`Instant`]s are subtracted in the wrong order.
///
/// Because the BIOS provides only a [`u32`] tick counter, [`Instant`] uses a [`u64`] cumulative
/// tick count internally to preserve monotonicity — which would overflow after roughly
/// 32 billion years and break monotonicity.
///
/// [`now`]: Instant::now
/// [`duration_since`]: Instant::duration_since
/// [`elapsed`]: Instant::elapsed
/// [`sub`]: Instant::sub
/// [`checked_duration_since`]: Instant::checked_duration_since
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    ticks: u64,
}

impl Instant {
    /// Returns an [`Instant`] corresponding to “[`now`]”.
    ///
    /// [`now`]: Instant::now
    ///
    /// # Examples
    ///
    /// ```
    /// use dos_std::time::Instant;
    ///
    /// let now = Instant::now();
    /// ```
    #[must_use]
    pub fn now() -> Self {
        let (hi, lo): (u16, u16);
        unsafe { asm!("int 0x1A", in("ah") 0u8, lateout("cx") hi, lateout("dx") lo, options(nomem, nostack)) };

        let now = (hi as u32) << 16 | lo as u32;

        if now < unsafe { TICKS } {
            unsafe { OFFSET = OFFSET.wrapping_add(0x1800B0) }
        }

        unsafe { TICKS = now };

        Self { ticks: unsafe { OFFSET } + now as u64 }
    }

    /// Returns the amount of time elapsed from another instant to this one,
    /// or zero duration if that instant is later than this one.
    #[must_use]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// Returns the amount of time elapsed from another instant to this one,
    /// or None if that instant is later than this one.
    ///
    /// Due to [monotonicity bugs], even under correct logical ordering of the passed [`Instant`]s,
    /// this method can return [`None`].
    ///
    /// [monotonicity bugs]: Instant#monotonicity
    #[must_use]
    pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
        self.ticks.checked_sub(earlier.ticks).map(|ticks| Duration::new(ticks / 18, ((ticks % 18) * 55_000_000) as u32))
    }

    /// Returns the amount of time elapsed from another [`Instant`] to this one,
    /// or zero [`Duration`] if that [`Instant`] is later than this one.
    #[must_use]
    pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// Returns the amount of time elapsed since this [`Instant`].
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// [`Instant`] (which means it's inside the bounds of the underlying data structure), [`None`]
    /// otherwise.
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        let ticks = duration.as_secs().saturating_mul(18) + (duration.subsec_nanos() as u64 / 55_000_000);
        self.ticks.checked_add(ticks).map(|ticks| Self { ticks })
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), [`None`]
    /// otherwise.
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let ticks = duration.as_secs().saturating_mul(18) + (duration.subsec_nanos() as u64 / 55_000_000);
        self.ticks.checked_sub(ticks).map(|ticks| Self { ticks })
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure. See [`Instant::checked_add`] for a version without panic.
    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add(rhs).expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.duration_since(rhs)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        self.checked_sub(rhs).expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}
