static DOSYEAR_TO_BCD: [u16; 128] = [
    // 1980s
    0x1980, 0x1981, 0x1982, 0x1983, 0x1984, 0x1985, 0x1986, 0x1987, 0x1988, 0x1989,
    // 1990s
    0x1990, 0x1991, 0x1992, 0x1993, 0x1994, 0x1995, 0x1996, 0x1997, 0x1998, 0x1999,
    // 2000s
    0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005, 0x2006, 0x2007, 0x2008, 0x2009,
    // 2010s
    0x2010, 0x2011, 0x2012, 0x2013, 0x2014, 0x2015, 0x2016, 0x2017, 0x2018, 0x2019,
    // 2020s
    0x2020, 0x2021, 0x2022, 0x2023, 0x2024, 0x2025, 0x2026, 0x2027, 0x2028, 0x2029,
    // 2030s
    0x2030, 0x2031, 0x2032, 0x2033, 0x2034, 0x2035, 0x2036, 0x2037, 0x2038, 0x2039,
    // 2040s
    0x2040, 0x2041, 0x2042, 0x2043, 0x2044, 0x2045, 0x2046, 0x2047, 0x2048, 0x2049,
    // 2050s
    0x2050, 0x2051, 0x2052, 0x2053, 0x2054, 0x2055, 0x2056, 0x2057, 0x2058, 0x2059,
    // 2060s
    0x2060, 0x2061, 0x2062, 0x2063, 0x2064, 0x2065, 0x2066, 0x2067, 0x2068, 0x2069,
    // 2070s
    0x2070, 0x2071, 0x2072, 0x2073, 0x2074, 0x2075, 0x2076, 0x2077, 0x2078, 0x2079,
    // 2080s
    0x2080, 0x2081, 0x2082, 0x2083, 0x2084, 0x2085, 0x2086, 0x2087, 0x2088, 0x2089,
    // 2090s
    0x2090, 0x2091, 0x2092, 0x2093, 0x2094, 0x2095, 0x2096, 0x2097, 0x2098, 0x2099,
    // 2100s
    0x2100, 0x2101, 0x2102, 0x2103, 0x2104, 0x2105, 0x2106, 0x2107,
];

pub fn dosyear_to_binary_coded_decimal(dosyear: u8) -> u16 {
    let uvii: u8 = dosyear & 0x7f;
    DOSYEAR_TO_BCD[uvii as usize]
}

#[cfg(feature = "dosyear2bcd")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn dosyear2bcd(dosyear: u8) -> u16 {
    dosyear_to_binary_coded_decimal(dosyear)
}

/// Converts year(1980 - 2107) to bcd(0x1980 - 0x2107).
///
/// ```plain
/// year: 1980, 1981, 1982, ... , 2107
/// year & 0xfff: 1980, 1981, 1982, ... , 2107
/// (year & 0xfff) + 68: 2048, 2049, 2050, ... , 2175
/// ((year & 0xfff) + 68) & 0x7f: 0, 1, 2, ... , 127
/// ```
pub fn year_to_binary_coded_decimal_1980_2107(year: u16) -> u16 {
    let bound: u16 = year & 0xfff; //  0, 1980, 2107, 4095
    let add: u16 = bound + 68; // 68, 2048, 2175, 4163
    let and: u16 = add & 0x7f; // 68,    0,  127,   67
    dosyear_to_binary_coded_decimal(and as u8)
}

#[cfg(feature = "year2bcd")]
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn year2bcd(year_1980_2107: u16) -> u16 {
    year_to_binary_coded_decimal_1980_2107(year_1980_2107)
}

#[cfg(test)]
mod test {
    mod year_to_binary_coded_decimal_1980_2107 {
        macro_rules! create_test {
            ($fname: ident, $input: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let got: u16 = crate::year_to_binary_coded_decimal_1980_2107($input);
                    assert_eq!(got, $expected);
                }
            };
        }

        create_test!(y1980, 1980, 0x1980);
        create_test!(y1981, 1981, 0x1981);
        create_test!(y1982, 1982, 0x1982);

        create_test!(y1990, 1990, 0x1990);
        create_test!(y1991, 1991, 0x1991);
        create_test!(y1992, 1992, 0x1992);

        create_test!(y2000, 2000, 0x2000);
        create_test!(y2001, 2001, 0x2001);
        create_test!(y2002, 2002, 0x2002);

        create_test!(y2010, 2010, 0x2010);
        create_test!(y2020, 2020, 0x2020);
        create_test!(y2030, 2030, 0x2030);
        create_test!(y2040, 2040, 0x2040);
        create_test!(y2050, 2050, 0x2050);
        create_test!(y2060, 2060, 0x2060);
        create_test!(y2070, 2070, 0x2070);
        create_test!(y2080, 2080, 0x2080);
        create_test!(y2090, 2090, 0x2090);
        create_test!(y2100, 2100, 0x2100);

        create_test!(y2107, 2107, 0x2107);
    }
}
