use core::fmt::{Debug, Display};

/// An CP437 character.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CP437Char {
    /// `'\0'`
    Null = 0,
    /// `'☺'`
    WhiteSmilingFace = 1,
    /// `'☻'`
    BlackSmilingFace = 2,
    /// `'♥'`
    BlackHeartSuit = 3,
    /// `'♦'`
    BlackDiamondSuit = 4,
    /// `'♣︎'`
    BlackClubSuit = 5,
    /// `'♠︎'`
    BlackSpadeSuit = 6,
    /// `'•'`
    Bullet = 7,
    /// `'◘'`
    InverseBullet = 8,
    /// `'○'`
    WhiteCircle = 9,
    /// `'◙'`
    InverseWhiteCircle = 10,
    /// `'♂︎'`
    MaleSign = 11,
    /// `'♀︎'`
    FemaleSign = 12,
    /// `'♪'`
    EighthNote = 13,
    /// `'♫'`
    BeamedEighthNotes = 14,
    /// `'☼'`
    WhiteSunWithRays = 15,
    /// `'►'`
    BlackRightPointingPointer = 16,
    /// `'◄'`
    BlackLeftPointingPointer = 17,
    /// `'↕'`
    UpDownArrow = 18,
    /// `'‼'`
    DoubleExclamationMark = 19,
    /// `'¶'`
    PilcrowSign = 20,
    /// `'§'`
    SectionSign = 21,
    /// `'▬'`
    BlackRectangle = 22,
    /// `'↨'`
    UpDownArrowWithBase = 23,
    /// `'↑'`
    UpwardsArrow = 24,
    /// `'↓'`
    DownwardsArrow = 25,
    /// `'→'`
    RightwardsArrow = 26,
    /// `'←'`
    LeftwardsArrow = 27,
    /// `'∟'`
    RightAngle = 28,
    /// `'↔'`
    LeftRightArrow = 29,
    /// `'▲'`
    BlackUpPointingTriangle = 30,
    /// `'▼'`
    BlackDownPointingTriangle = 31,
    /// `' '`
    Space = 32,
    /// `'!'`
    ExclamationMark = 33,
    /// `'"'`
    QuotationMark = 34,
    /// `'#'`
    NumberSign = 35,
    /// `'$'`
    DollarSign = 36,
    /// `'%'`
    PercentSign = 37,
    /// `'&'`
    Ampersand = 38,
    /// `'''`
    Apostrophe = 39,
    /// `'('`
    LeftParenthesis = 40,
    /// `')'`
    RightParenthesis = 41,
    /// `'*'`
    Asterisk = 42,
    /// `'+'`
    PlusSign = 43,
    /// `','`
    Comma = 44,
    /// `'-'`
    HyphenMinus = 45,
    /// `'.'`
    FullStop = 46,
    /// `'/'`
    Solidus = 47,
    /// `'0'`
    DigitZero = 48,
    /// `'1'`
    DigitOne = 49,
    /// `'2'`
    DigitTwo = 50,
    /// `'3'`
    DigitThree = 51,
    /// `'4'`
    DigitFour = 52,
    /// `'5'`
    DigitFive = 53,
    /// `'6'`
    DigitSix = 54,
    /// `'7'`
    DigitSeven = 55,
    /// `'8'`
    DigitEight = 56,
    /// `'9'`
    DigitNine = 57,
    /// `':'`
    Colon = 58,
    /// `';'`
    Semicolon = 59,
    /// `'<'`
    LessThanSign = 60,
    /// `'='`
    EqualsSign = 61,
    /// `'>'`
    GreaterThanSign = 62,
    /// `'?'`
    QuestionMark = 63,
    /// `'@'`
    CommercialAt = 64,
    /// `'A'`
    LatinCapitalLetterA = 65,
    /// `'B'`
    LatinCapitalLetterB = 66,
    /// `'C'`
    LatinCapitalLetterC = 67,
    /// `'D'`
    LatinCapitalLetterD = 68,
    /// `'E'`
    LatinCapitalLetterE = 69,
    /// `'F'`
    LatinCapitalLetterF = 70,
    /// `'G'`
    LatinCapitalLetterG = 71,
    /// `'H'`
    LatinCapitalLetterH = 72,
    /// `'I'`
    LatinCapitalLetterI = 73,
    /// `'J'`
    LatinCapitalLetterJ = 74,
    /// `'K'`
    LatinCapitalLetterK = 75,
    /// `'L'`
    LatinCapitalLetterL = 76,
    /// `'M'`
    LatinCapitalLetterM = 77,
    /// `'N'`
    LatinCapitalLetterN = 78,
    /// `'O'`
    LatinCapitalLetterO = 79,
    /// `'P'`
    LatinCapitalLetterP = 80,
    /// `'Q'`
    LatinCapitalLetterQ = 81,
    /// `'R'`
    LatinCapitalLetterR = 82,
    /// `'S'`
    LatinCapitalLetterS = 83,
    /// `'T'`
    LatinCapitalLetterT = 84,
    /// `'U'`
    LatinCapitalLetterU = 85,
    /// `'V'`
    LatinCapitalLetterV = 86,
    /// `'W'`
    LatinCapitalLetterW = 87,
    /// `'X'`
    LatinCapitalLetterX = 88,
    /// `'Y'`
    LatinCapitalLetterY = 89,
    /// `'Z'`
    LatinCapitalLetterZ = 90,
    /// `'['`
    LeftSquareBracket = 91,
    /// `'\\'`
    ReverseSolidus = 92,
    /// `']'`
    RightSquareBracket = 93,
    /// `'^'`
    CircumflexAccent = 94,
    /// `'_'`
    LowLine = 95,
    /// `'`'`
    GraveAccent = 96,
    /// `'a'`
    LatinSmallLetterA = 97,
    /// `'b'`
    LatinSmallLetterB = 98,
    /// `'c'`
    LatinSmallLetterC = 99,
    /// `'d'`
    LatinSmallLetterD = 100,
    /// `'e'`
    LatinSmallLetterE = 101,
    /// `'f'`
    LatinSmallLetterF = 102,
    /// `'g'`
    LatinSmallLetterG = 103,
    /// `'h'`
    LatinSmallLetterH = 104,
    /// `'i'`
    LatinSmallLetterI = 105,
    /// `'j'`
    LatinSmallLetterJ = 106,
    /// `'k'`
    LatinSmallLetterK = 107,
    /// `'l'`
    LatinSmallLetterL = 108,
    /// `'m'`
    LatinSmallLetterM = 109,
    /// `'n'`
    LatinSmallLetterN = 110,
    /// `'o'`
    LatinSmallLetterO = 111,
    /// `'p'`
    LatinSmallLetterP = 112,
    /// `'q'`
    LatinSmallLetterQ = 113,
    /// `'r'`
    LatinSmallLetterR = 114,
    /// `'s'`
    LatinSmallLetterS = 115,
    /// `'t'`
    LatinSmallLetterT = 116,
    /// `'u'`
    LatinSmallLetterU = 117,
    /// `'v'`
    LatinSmallLetterV = 118,
    /// `'w'`
    LatinSmallLetterW = 119,
    /// `'x'`
    LatinSmallLetterX = 120,
    /// `'y'`
    LatinSmallLetterY = 121,
    /// `'z'`
    LatinSmallLetterZ = 122,
    /// `'{'`
    LeftCurlyBracket = 123,
    /// `'|'`
    VerticalLine = 124,
    /// `'}'`
    RightCurlyBracket = 125,
    /// `'~'`
    Tilde = 126,
    /// `'⌂'`
    House = 127,
    /// `'Ç'`
    LatinCapitalLetterCWithCedilla = 128,
    /// `'ü'`
    LatinSmallLetterUWithDiaeresis = 129,
    /// `'é'`
    LatinSmallLetterEWithAcute = 130,
    /// `'â'`
    LatinSmallLetterAWithCircumflex = 131,
    /// `'ä'`
    LatinSmallLetterAWithDiaeresis = 132,
    /// `'à'`
    LatinSmallLetterAWithGrave = 133,
    /// `'å'`
    LatinSmallLetterAWithRingAbove = 134,
    /// `'ç'`
    LatinSmallLetterCWithCedilla = 135,
    /// `'ê'`
    LatinSmallLetterEWithCircumflex = 136,
    /// `'ë'`
    LatinSmallLetterEWithDiaeresis = 137,
    /// `'è'`
    LatinSmallLetterEWithGrave = 138,
    /// `'ï'`
    LatinSmallLetterIWithDiaeresis = 139,
    /// `'î'`
    LatinSmallLetterIWithCircumflex = 140,
    /// `'ì'`
    LatinSmallLetterIWithGrave = 141,
    /// `'Ä'`
    LatinCapitalLetterAWithDiaeresis = 142,
    /// `'Å'`
    LatinCapitalLetterAWithRingAbove = 143,
    /// `'É'`
    LatinCapitalLetterEWithAcute = 144,
    /// `'æ'`
    LatinSmallLetterAe = 145,
    /// `'Æ'`
    LatinCapitalLetterAe = 146,
    /// `'ô'`
    LatinSmallLetterOWithCircumflex = 147,
    /// `'ö'`
    LatinSmallLetterOWithDiaeresis = 148,
    /// `'ò'`
    LatinSmallLetterOWithGrave = 149,
    /// `'û'`
    LatinSmallLetterUWithCircumflex = 150,
    /// `'ù'`
    LatinSmallLetterUWithGrave = 151,
    /// `'ÿ'`
    LatinSmallLetterYWithDiaeresis = 152,
    /// `'Ö'`
    LatinCapitalLetterOWithDiaeresis = 153,
    /// `'Ü'`
    LatinCapitalLetterUWithDiaeresis = 154,
    /// `'¢'`
    CentSign = 155,
    /// `'£'`
    PoundSign = 156,
    /// `'¥'`
    YenSign = 157,
    /// `'₧'`
    PesetaSign = 158,
    /// `'ƒ'`
    LatinSmallLetterFWithHook = 159,
    /// `'á'`
    LatinSmallLetterAWithAcute = 160,
    /// `'í'`
    LatinSmallLetterIWithAcute = 161,
    /// `'ó'`
    LatinSmallLetterOWithAcute = 162,
    /// `'ú'`
    LatinSmallLetterUWithAcute = 163,
    /// `'ñ'`
    LatinSmallLetterNWithTilde = 164,
    /// `'Ñ'`
    LatinCapitalLetterNWithTilde = 165,
    /// `'ª'`
    FeminineOrdinalIndicator = 166,
    /// `'º'`
    MasculineOrdinalIndicator = 167,
    /// `'¿'`
    InvertedQuestionMark = 168,
    /// `'⌐'`
    ReversedNotSign = 169,
    /// `'¬'`
    NotSign = 170,
    /// `'½'`
    VulgarFractionOneHalf = 171,
    /// `'¼'`
    VulgarFractionOneQuarter = 172,
    /// `'¡'`
    InvertedExclamationMark = 173,
    /// `'«'`
    LeftPointingDoubleAngleQuotationMark = 174,
    /// `'»'`
    RightPointingDoubleAngleQuotationMark = 175,
    /// `'░'`
    LightShade = 176,
    /// `'▒'`
    MediumShade = 177,
    /// `'▓'`
    DarkShade = 178,
    /// `'│'`
    BoxDrawingsLightVertical = 179,
    /// `'┤'`
    BoxDrawingsLightVerticalAndLeft = 180,
    /// `'╡'`
    BoxDrawingsVerticalSingleAndLeftDouble = 181,
    /// `'╢'`
    BoxDrawingsVerticalDoubleAndLeftSingle = 182,
    /// `'╖'`
    BoxDrawingsDownDoubleAndLeftSingle = 183,
    /// `'╕'`
    BoxDrawingsDownSingleAndLeftDouble = 184,
    /// `'╣'`
    BoxDrawingsDoubleVerticalAndLeft = 185,
    /// `'║'`
    BoxDrawingsDoubleVertical = 186,
    /// `'╗'`
    BoxDrawingsDoubleDownAndLeft = 187,
    /// `'╝'`
    BoxDrawingsDoubleUpAndLeft = 188,
    /// `'╜'`
    BoxDrawingsUpDoubleAndLeftSingle = 189,
    /// `'╛'`
    BoxDrawingsUpSingleAndLeftDouble = 190,
    /// `'┐'`
    BoxDrawingsLightDownAndLeft = 191,
    /// `'└'`
    BoxDrawingsLightUpAndRight = 192,
    /// `'┴'`
    BoxDrawingsLightUpAndHorizontal = 193,
    /// `'┬'`
    BoxDrawingsLightDownAndHorizontal = 194,
    /// `'├'`
    BoxDrawingsLightVerticalAndRight = 195,
    /// `'─'`
    BoxDrawingsLightHorizontal = 196,
    /// `'┼'`
    BoxDrawingsLightVerticalAndHorizontal = 197,
    /// `'╞'`
    BoxDrawingsVerticalSingleAndRightDouble = 198,
    /// `'╟'`
    BoxDrawingsVerticalDoubleAndRightSingle = 199,
    /// `'╚'`
    BoxDrawingsDoubleUpAndRight = 200,
    /// `'╔'`
    BoxDrawingsDoubleDownAndRight = 201,
    /// `'╩'`
    BoxDrawingsDoubleUpAndHorizontal = 202,
    /// `'╦'`
    BoxDrawingsDoubleDownAndHorizontal = 203,
    /// `'╠'`
    BoxDrawingsDoubleVerticalAndRight = 204,
    /// `'═'`
    BoxDrawingsDoubleHorizontal = 205,
    /// `'╬'`
    BoxDrawingsDoubleVerticalAndHorizontal = 206,
    /// `'╧'`
    DrawingsUpSingleAndHorizontalDouble = 207,
    /// `'╨'`
    BoxDrawingsUpDoubleAndHorizontalSingle = 208,
    /// `'╤'`
    BoxDrawingsDownSingleAndHorizontalDouble = 209,
    /// `'╥'`
    BoxDrawingsDownDoubleAndHorizontalSingle = 210,
    /// `'╙'`
    BoxDrawingsUpDoubleAndRightSingle = 211,
    /// `'╘'`
    BoxDrawingsUpSingleAndRightDouble = 212,
    // `'╒'`
    BoxDrawingsDownSingleAndRightDouble = 213,
    // `'╓'`
    DrawingsDownDoubleAndRightSingle = 214,
    /// `'╫'`
    BoxDrawingsVerticalDoubleAndHorizontalSingle = 215,
    /// `'╪'`
    BoxDrawingsVerticalSingleAndHorizontalDouble = 216,
    /// `'┘'`
    BoxDrawingsLightUpAndLeft = 217,
    /// `'┌'`
    BoxDrawingsLightDownAndRight = 218,
    /// `'█'`
    FullBlock = 219,
    /// `'▄'`
    LowerHalfBlock = 220,
    /// `'▌'`
    LeftHalfBlock = 221,
    /// `'▐'`
    RightHalfBlock = 222,
    /// `'▀'`
    UpperHalfBlock = 223,
    /// `'α'`
    GreekSmallLetterAlpha = 224,
    /// `'ß'`
    LatinSmallLetterSharpS = 225,
    /// `'Γ'`
    GreekCapitalLetterGamma = 226,
    /// `'π'`
    GreekSmallLetterPi = 227,
    /// `'Σ'`
    GreekCapitalLetterSigma = 228,
    /// `'σ'`
    GreekSmallLetterSigma = 229,
    /// `'µ'`
    MicroSign = 230,
    /// `'τ'`
    GreekSmallLetterTau = 231,
    /// `'Φ'`
    CapitalLetterPhi = 232,
    /// `'Θ'`
    GreekCapitalLetterTheta = 233,
    /// `'Ω'`
    GreekCapitalLetterOmega = 234,
    /// `'δ'`
    GreekSmallLetterDelta = 235,
    /// `'∞'`
    Infinity = 236,
    /// `'φ'`
    SmallLetterPhi = 237,
    /// `'ε'`
    GreekSmallLetterEpsilon = 238,
    /// `'∩'`
    Intersection = 239,
    /// `'≡'`
    IdenticalTo = 240,
    /// `'±'`
    PlusMinusSign = 241,
    /// `'≥'`
    GreaterThanOrEqualTo = 242,
    /// `'≤'`
    LessThanOrEqualTo = 243,
    /// `'⌠'`
    TopHalfIntegral = 244,
    /// `'⌡'`
    BottomHalfIntegral = 245,
    /// `'÷'`
    DivisionSign = 246,
    /// `'≈'`
    AlmostEqualTo = 247,
    /// `'°'`
    DegreeSign = 248,
    /// `'∙'`
    BulletOperator = 249,
    /// `'·'`
    MiddleDot = 250,
    /// `'√'`
    SquareRoot = 251,
    /// `'ⁿ'`
    SuperscriptLatinSmallLetterN = 252,
    /// `'²'`
    SuperscriptTwo = 253,
    /// `'■'`
    BlackSquare = 254,
    /// `' '`
    NoBreakSpace = 255,
}

pub trait CodePage: Sized {
    const TABLE: [char; 256];
}

impl CodePage for CP437Char {
    const TABLE: [char; 256] = [
        '\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←',
        '∟', '↔', '▲', '▼', ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', 'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï',
        'î', 'ì', 'Ä', 'Å', 'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', 'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º',
        '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐', '└', '┴', '┬', '├',
        '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀',
        'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√',
        'ⁿ', '²', '■', '\u{A0}',
    ];
}

impl CP437Char {
    #[must_use]
    pub const fn new(ch: char) -> Option<Self> {
        if matches!(ch, ' '..='~') {
            return Some(unsafe { core::mem::transmute(ch as u8) });
        }

        let mut index = 0;
        while index < <Self as CodePage>::TABLE.len() {
            if <Self as CodePage>::TABLE[index] == ch {
                return Some(unsafe { core::mem::transmute(index as u8) });
            }
            index += 1;
        }

        None
    }

    /// Converts a CP437 character into a `u8`.
    #[inline]
    #[must_use]
    pub const fn as_byte(self) -> u8 {
        self as u8
    }

    /// Converts a CP437 character into a `char`.
    #[inline]
    #[must_use]
    pub const fn as_char(self) -> char {
        self as u8 as char
    }
}

impl Display for CP437Char {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.as_char(), f)
    }
}

impl Debug for CP437Char {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.as_char(), f)
    }
}

impl Default for CP437Char {
    #[inline]
    fn default() -> Self {
        Self::Null
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CP437Str {
    slice: [CP437Char],
}

impl CP437Str {
    pub fn new<'a>(input: &str, buf: &'a mut [CP437Char]) -> &'a CP437Str {
        let mut i = 0;
        for ch in input.chars() {
            if i >= buf.len() {
                break;
            }

            buf[i] = CP437Char::new(ch).unwrap_or_else(|| unsafe { core::mem::transmute(b'?') });

            i += 1;
        }

        unsafe { CP437Str::from_slice_unchecked(&buf[..i]) }
    }

    pub const unsafe fn from_slice_unchecked(slice: &[CP437Char]) -> &Self {
        unsafe { &*(slice as *const [CP437Char] as *const CP437Str) }
    }

    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) }
    }

    /// Returns the entire string as slice of `CP437Char`s.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[CP437Char] {
        &self.slice
    }

    /// Returns the entire string as mutable slice of `CP437Char`s.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [CP437Char] {
        &mut self.slice
    }

    #[inline]
    #[must_use]
    pub const fn as_ptr(&self) -> *const CP437Char {
        self.as_slice().as_ptr()
    }

    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut CP437Char {
        self.as_mut_slice().as_mut_ptr()
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.slice.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
