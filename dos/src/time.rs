//! Temporal quantification.

use core::arch::asm;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use thiserror::Error;

pub use core::time::Duration;

/// A measurement of a monotonically nondecreasing clock.
/// Opaque and useful only with [`Duration`].
/// 
/// ## Monotonicity
///
/// On all platforms [`Instant`] will try to use an OS API that guarantees monotonic behavior
/// if available, which is the case for all [tier 1] platforms.
/// In practice such guarantees are – under rare circumstances – broken by hardware, virtualization
/// or operating system bugs. To work around these bugs and platforms not offering monotonic clocks
/// [`duration_since`], [`elapsed`] and [`sub`] saturate to zero. In older Rust versions this
/// lead to a panic instead. [`checked_duration_since`] can be used to detect and handle situations
/// where monotonicity is violated, or `Instant`s are subtracted in the wrong order.
/// 
/// [tier 1]: https://doc.rust-lang.org/rustc/platform-support.html
/// [`duration_since`]: Instant::duration_since
/// [`elapsed`]: Instant::elapsed
/// [`sub`]: Instant::sub
/// [`checked_duration_since`]: Instant::checked_duration_since
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    /// BIOS ticks since midnight.
    ticks: u32,
    overflowed: bool,
}

impl Instant {
    /// Returns an instant corresponding to "now".
    ///
    /// # Examples
    /// ```
    /// use dos::time::Instant;
    ///
    /// let now = Instant::now();
    /// ```
    #[must_use]
    pub fn now() -> Self {
        let (overflowed, hi, lo): (u8, u16, u16);
        unsafe { asm!("int 0x1A", in("ah") 0u8, out("al") overflowed, out("cx") hi, out("dx") lo, options(nomem, nostack)) }
        Self { ticks: ((hi as u32) << 16) | (lo as u32), overflowed: overflowed != 0 }
    }

    /// Returns the amount of time elapsed from another instant to this one,
    /// or zero duration if that instant is later than this one.
    #[must_use]
    pub fn duration_since(&self, earlier: Self) -> Duration {
        todo!()
    }

    /// Returns the amount of time elapsed from another instant to this one,
    /// or None if that instant is later than this one.
    /// 
    /// Due to [monotonicity bugs], even under correct logical ordering of the passed `Instant`s,
    /// this method can return `None`.
    ///
    /// [monotonicity bugs]: Instant#monotonicity
    #[must_use]
    pub fn checked_duration_since(&self, earlier: Self) -> Duration {
        todo!()
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// [`Instant`] (which means it's inside the bounds of the underlying data structure), [`None`]
    /// otherwise.
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        let ticks_to_add = duration.as_secs().checked_mul(18)?.checked_add(duration.subsec_nanos() as u64 / 55_000_000)?;
        let new_ticks = self.ticks.checked_add(ticks_to_add as u32)?;

        Some(Self { ticks: new_ticks, overflowed: self.overflowed })
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// [`Instant`] (which means it's inside the bounds of the underlying data structure), [`None`]
    /// otherwise.
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let ticks_to_sub = duration.as_secs().checked_mul(18)?.checked_add((duration.subsec_nanos() as u64) / 55_000_000)?;
        let new_ticks = self.ticks.checked_sub(ticks_to_sub as u32)?;

        Some(Self {
            ticks: new_ticks,
            overflowed: self.overflowed,
        })
    }

    /// Returns the amount of time elapsed since this instant.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Instant::now() - *self
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
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Instant {
    type Output = Duration;

    /// Returns the amount of time elapsed from another instant to this one,
    /// or zero duration if that instant is later than this one.
    ///
    /// # Panics
    ///
    /// Previous Rust versions panicked when `other` was later than `self`. Currently this
    /// method saturates. Future versions may reintroduce the panic in some circumstances.
    /// See [Monotonicity].
    ///
    /// [Monotonicity]: Instant#monotonicity
    fn sub(self, rhs: Self) -> Self::Output {
        self.duration_since(rhs)
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub(rhs).expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

/// A measurement of the system clock, useful for talking to
/// external entities like the file system or other processes.
///
/// Distinct from the [`Instant`] type, this time measurement **is not
/// monotonic**. This means that you can save a file to the file system, then
/// save another file to the file system, **and the second file has a
/// [`SystemTime`] measurement earlier than the first**. In other words, an
/// operation that happens after another operation in real time may have an
/// earlier [`SystemTime`]!
///
/// Consequently, comparing two [`SystemTime`] instances to learn about the
/// duration between them returns a [`Result`] instead of an infallible [`Duration`]
/// to indicate that this sort of time drift may happen and needs to be handled.
///
/// Although a [`SystemTime`] cannot be directly inspected, the [`UNIX_EPOCH`]
/// constant is provided in this module as an anchor in time to learn
/// information about a [`SystemTime`]. By calculating the duration from this
/// fixed point in time, a `SystemTime` can be converted to a human-readable time,
/// or perhaps some other string representation.
///
/// A [`SystemTime`] does not count leap seconds.
/// `SystemTime::now()`'s behavior around a leap second
/// is the same as the operating system's wall clock.
/// The precise behavior near a leap second
/// (e.g. whether the clock appears to run slow or fast, or stop, or jump)
/// depends on platform and configuration,
/// so should not be relied on.
///
/// [`UNIX_EPOCH`]: SystemTime::UNIX_EPOCH
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemTime {
    unix: u32,
}

impl SystemTime {
    /// An anchor in time which can be used to create new [`SystemTime`] instances or
    /// learn about where in time a [`SystemTime`] lies.
    pub const UNIX_EPOCH: Self = Self { unix: 0 };

    /// Returns the system time corresponding to “now”.
    ///
    /// # Examples
    /// ```
    /// use dos::time::SystemTime;
    ///
    /// let time = SystemTime::now();
    /// ```
    #[must_use]
    pub fn now() -> Self {
        let (h, m, s): (u8, u8, u8);
        unsafe { asm!("int 0x21", in("ah") 0x2Cu8, lateout("ch") h, lateout("cl") m, lateout("dh") s, options(nomem, nostack)) };
        let dos = (h as u32) * 3600 + (m as u32) * 60 + (s as u32);
        Self { unix: 315_532_800 + dos }
    }

    /// Returns <code>[Some]\(t)</code> where `t` is the time `self + duration` if `t` can be represented as
    /// [`SystemTime`] (which means it's inside the bounds of the underlying data structure), [`None`]
    /// otherwise.
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.unix.checked_add(duration.as_secs() as u32).map(|unix| Self { unix })
    }

    /// Returns <code>[Some]\(t)</code> where `t` is the time `self - duration` if `t` can be represented as
    /// [`SystemTime`] (which means it's inside the bounds of the underlying data structure), [`None`]
    /// otherwise.
    pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime> {
        self.unix.checked_sub(duration.as_secs() as u32).map(|unix| Self { unix })
    }

    /// Returns the amount of time elapsed from an earlier point in time.
    ///
    /// This function may fail because measurements taken earlier are not
    /// guaranteed to always be before later measurements (due to anomalies such
    /// as the system clock being adjusted either forwards or backwards).
    /// [`Instant`] can be used to measure elapsed time without this risk of failure.
    ///
    /// If successful, <code>[Ok]\([Duration])</code> is returned where the duration represents
    /// the amount of time elapsed from the specified measurement to this one.
    ///
    /// Returns an [`Err`] if earlier is later than self, and
    /// the error contains how far from self the time is.
    ///
    /// # Examples
    /// ```no_run
    /// use dos::time::SystemTime;
    ///
    /// let time = SystemTime::now();
    /// let new_time = SystemTime::now();
    /// let difference = new_time.duration_since(time)
    ///     .expect("Clock may have gone backwards");
    /// println!("{difference:?}")
    /// ```
    pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
        if self.unix >= earlier.unix {
            Ok(Duration::from_secs((self.unix - earlier.unix) as u64))
        } else {
            Err(SystemTimeError(Duration::from_secs((earlier.unix - self.unix) as u64)))
        }
    }

    /// Returns the difference from this system time to the current clock time.
    ///
    /// This function may fail as the underlying system clock is susceptible to
    /// drift and updates (e.g., the system clock could go backwards), so this
    /// function might not always succeed. If successful, <code>[Ok]\([Duration])</code> is
    /// returned where the duration represents the amount of time elapsed from
    /// this time measurement to the current time.
    ///
    /// To measure elapsed time reliably, use [`Instant`] instead.
    ///
    /// Returns an [`Err`] if `self` is later than the current system time, and
    /// the error contains how far from the current system time `self` is.
    pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
        Self::now().duration_since(*self)
    }
}

impl Add<Duration> for SystemTime {
    type Output = Self;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure. See [`SystemTime::checked_add`] for a version without panic.
    fn add(self, duration: Duration) -> Self::Output {
        self.checked_add(duration).expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, duration: Duration) -> Self::Output {
        self.checked_sub(duration).expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

/// An error returned from the [`duration_since`] and [`elapsed`] methods on
/// [`SystemTime`], used to learn how far in the opposite direction a system time
/// lies.
#[derive(Error, Debug, Clone)]
#[error("second time provided was later than self")]
pub struct SystemTimeError(Duration);

impl SystemTimeError {
    /// Returns the positive duration which represents how far forward the
    /// second system time was from the first.
    ///
    /// A `SystemTimeError` is returned from the [`SystemTime::duration_since`]
    /// and [`SystemTime::elapsed`] methods whenever the second system time
    /// represents a point later in time than the `self` of the method call.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.0
    }
}
