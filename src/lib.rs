/// Converts letters of the Russian alphabet into Latin characters similar in outline.
///
/// # Examples
///
/// ```
/// # use pseudo_cyrillic::convert;
///
/// assert_eq!(convert("Воин Света"), "BouH CBema");
/// ```
pub fn convert(original: &str) -> String {
    let mut new_string = String::with_capacity(original.len());

    for ch in original.chars() {
        let analog = match ch {
            // Строчные буквы
            'а' => "a",
            'б' => "6",
            'в' => "B",
            'г' => "r",
            'д' => "D",
            'е' => "e",
            'ё' => "e",
            'ж' => "zh",
            'з' => "3",
            'и' => "u",
            'й' => "u",
            'к' => "k",
            'л' => "JI",
            'м' => "M",
            'н' => "H",
            'о' => "o",
            'п' => "n",
            'р' => "p",
            'с' => "c",
            'т' => "m",
            'у' => "y",
            'ф' => "qp",
            'х' => "x",
            'ц' => "u",
            'ч' => "4",
            'ш' => "LLI",
            'щ' => "LLj",
            'ъ' => "b",
            'ы' => "bI",
            'ь' => "b",
            'э' => "3",
            'ю' => "IO",
            'я' => "R",
            // ПРОПИСНЫЕ БУКВЫ
            'А' => "A",
            'Б' => "6",
            'В' => "B",
            'Г' => "r",
            'Д' => "D",
            'Е' => "E",
            'Ё' => "E",
            'Ж' => "ZH",
            'З' => "3",
            'И' => "u",
            'Й' => "u",
            'К' => "K",
            'Л' => "JI",
            'М' => "M",
            'Н' => "H",
            'О' => "O",
            'П' => "n",
            'Р' => "P",
            'С' => "C",
            'Т' => "T",
            'У' => "Y",
            'Ф' => "qp",
            'Х' => "X",
            'Ц' => "U",
            'Ч' => "4",
            'Ш' => "LLI",
            'Щ' => "LLj",
            'Ъ' => "b",
            'Ы' => "bI",
            'Ь' => "b",
            'Э' => "3",
            'Ю' => "IO",
            'Я' => "R",
            // Остальные символы оставляем без изменений
            _ => {
                new_string.push(ch);
                continue
            }
        };

        new_string.push_str(analog);
    }

    new_string
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert(
            "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ A1"),
                   "a6BrDeezh3uukJIMHonpcmyqpxu4LLILLjbbIb3IORA6BrDEEZH3uuKJIMHOnPCTYqpXU4LLILLjbbIb3IOR A1"
        );
    }

    #[test]
    fn test_a() {
        assert_eq!(convert("а"), "a");
    }

    #[test]
    fn test_b() {
        assert_eq!(convert("б"), "6");
    }

    #[test]
    fn test_v() {
        assert_eq!(convert("в"), "B");
    }

    #[test]
    fn test_g() {
        assert_eq!(convert("г"), "r");
    }

    #[test]
    fn test_d() {
        assert_eq!(convert("д"), "D");
    }

    #[test]
    fn test_e() {
        assert_eq!(convert("е"), "e");
    }

    #[test]
    fn test_yo() {
        assert_eq!(convert("ё"), "e");
    }

    #[test]
    fn test_zh() {
        assert_eq!(convert("ж"), "zh");
    }

    #[test]
    fn test_z() {
        assert_eq!(convert("з"), "3");
    }

    #[test]
    fn test_i() {
        assert_eq!(convert("и"), "u");
    }

    #[test]
    fn test_j() {
        assert_eq!(convert("й"), "u");
    }

    #[test]
    fn test_k() {
        assert_eq!(convert("к"), "k");
    }

    #[test]
    fn test_l() {
        assert_eq!(convert("л"), "JI");
    }

    #[test]
    fn test_m() {
        assert_eq!(convert("м"), "M");
    }

    #[test]
    fn test_n() {
        assert_eq!(convert("н"), "H");
    }

    #[test]
    fn test_o() {
        assert_eq!(convert("о"), "o");
    }

    #[test]
    fn test_p() {
        assert_eq!(convert("п"), "n");
    }

    #[test]
    fn test_r() {
        assert_eq!(convert("р"), "p");
    }

    #[test]
    fn test_s() {
        assert_eq!(convert("с"), "c");
    }

    #[test]
    fn test_t() {
        assert_eq!(convert("т"), "m");
    }

    #[test]
    fn test_u() {
        assert_eq!(convert("у"), "y");
    }

    #[test]
    fn test_f() {
        assert_eq!(convert("ф"), "qp");
    }

    #[test]
    fn test_x() {
        assert_eq!(convert("х"), "x");
    }

    #[test]
    fn test_c() {
        assert_eq!(convert("ц"), "u");
    }

    #[test]
    fn test_ch() {
        assert_eq!(convert("ч"), "4");
    }

    #[test]
    fn test_sh() {
        assert_eq!(convert("ш"), "LLI");
    }

    #[test]
    fn test_shh() {
        assert_eq!(convert("щ"), "LLj");
    }

    #[test]

    fn test_tverdyi_znak() {
        assert_eq!(convert("ъ"), "b");
    }

    #[test]
    fn test_yy() {
        assert_eq!(convert("ы"), "bI");
    }

    #[test]
    fn test_myagkiy_znak() {
        assert_eq!(convert("ь"), "b");
    }

    #[test]
    fn test_ee() {
        assert_eq!(convert("э"), "3");
    }

    #[test]
    fn test_yu() {
        assert_eq!(convert("ю"), "IO");
    }

    #[test]
    fn test_ya() {
        assert_eq!(convert("я"), "R");
    }

    #[test]
    fn test_A() {
        assert_eq!(convert("А"), "A");
    }

    #[test]
    fn test_B() {
        assert_eq!(convert("Б"), "6");
    }

    #[test]
    fn test_V() {
        assert_eq!(convert("В"), "B");
    }

    #[test]
    fn test_G() {
        assert_eq!(convert("Г"), "r");
    }

    #[test]
    fn test_D() {
        assert_eq!(convert("Д"), "D");
    }

    #[test]
    fn test_E() {
        assert_eq!(convert("Е"), "E");
    }

    #[test]
    fn test_Yo() {
        assert_eq!(convert("Ё"), "E");
    }

    #[test]
    fn test_Zh() {
        assert_eq!(convert("Ж"), "ZH");
    }

    #[test]
    fn test_Z() {
        assert_eq!(convert("З"), "3");
    }

    #[test]
    fn test_I() {
        assert_eq!(convert("И"), "u");
    }

    #[test]
    fn test_J() {
        assert_eq!(convert("Й"), "u");
    }

    #[test]
    fn test_K() {
        assert_eq!(convert("К"), "K");
    }

    #[test]
    fn test_L() {
        assert_eq!(convert("Л"), "JI");
    }

    #[test]
    fn test_M() {
        assert_eq!(convert("М"), "M");
    }

    #[test]
    fn test_N() {
        assert_eq!(convert("Н"), "H");
    }

    #[test]
    fn test_O() {
        assert_eq!(convert("О"), "O");
    }

    #[test]
    fn test_P() {
        assert_eq!(convert("П"), "n");
    }

    #[test]
    fn test_R() {
        assert_eq!(convert("Р"), "P");
    }

    #[test]
    fn test_S() {
        assert_eq!(convert("С"), "C");
    }

    #[test]
    fn test_T() {
        assert_eq!(convert("Т"), "T");
    }

    #[test]
    fn test_U() {
        assert_eq!(convert("У"), "Y");
    }

    #[test]
    fn test_F() {
        assert_eq!(convert("Ф"), "qp");
    }

    #[test]
    fn test_X() {
        assert_eq!(convert("Х"), "X");
    }

    #[test]
    fn test_C() {
        assert_eq!(convert("Ц"), "U");
    }

    #[test]
    fn test_Ch() {
        assert_eq!(convert("Ч"), "4");
    }

    #[test]
    fn test_Sh() {
        assert_eq!(convert("Ш"), "LLI");
    }

    #[test]
    fn test_Shh() {
        assert_eq!(convert("Щ"), "LLj");
    }

    #[test]
    fn test_Tverdiy_znak() {
        assert_eq!(convert("Ъ"), "b");
    }

    #[test]
    fn test_YY() {
        assert_eq!(convert("Ы"), "bI");
    }

    #[test]
    fn test_Myagkiy_znak() {
        assert_eq!(convert("Ь"), "b");
    }

    #[test]
    fn test_Ee() {
        assert_eq!(convert("Э"), "3");
    }

    #[test]
    fn test_Yu() {
        assert_eq!(convert("Ю"), "IO");
    }

    #[test]
    fn test_Ya() {
        assert_eq!(convert("Я"), "R");
    }
}
