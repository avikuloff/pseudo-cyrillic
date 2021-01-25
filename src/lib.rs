/// Converts letters of the Russian alphabet into Latin characters similar in outline.
///
/// # Examples
///
/// ```
/// # use pseudo_cyrillic::convert;
///
/// let nickname = "Воин Света".to_owned();
///
/// assert_eq!(convert(nickname), "BouH CBema");
/// ```
pub fn convert(original: String) -> String {
    let chars: Vec<char> = original.chars().collect();
    let mut result_string = String::new();

    for ch in chars.into_iter() {
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
                result_string.push(ch);
                continue
            }
        };

        result_string.push_str(analog);
    }

    result_string
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    #[ignore]
    fn test_convert() {
        assert_eq!(convert(
            "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".to_owned()),
                   "a6BrDeezh3uukJIMHonpcmyqpxu4LLILLjbbIb3IORA6BrDEEZH3uuKJIMHOnPCTYqpXU4LLILLjbbIb3IOR"
        );
    }

    #[test]
    fn test_a() {
        assert_eq!(convert("а".to_owned()), "a");
    }

    #[test]
    fn test_b() {
        assert_eq!(convert("б".to_owned()), "6");
    }

    #[test]
    fn test_v() {
        assert_eq!(convert("в".to_owned()), "B");
    }

    #[test]
    fn test_g() {
        assert_eq!(convert("г".to_owned()), "r");
    }

    #[test]
    fn test_d() {
        assert_eq!(convert("д".to_owned()), "D");
    }

    #[test]
    fn test_e() {
        assert_eq!(convert("е".to_owned()), "e");
    }

    #[test]
    fn test_yo() {
        assert_eq!(convert("ё".to_owned()), "e");
    }

    #[test]
    fn test_zh() {
        assert_eq!(convert("ж".to_owned()), "zh");
    }

    #[test]
    fn test_z() {
        assert_eq!(convert("з".to_owned()), "3");
    }

    #[test]
    fn test_i() {
        assert_eq!(convert("и".to_owned()), "u");
    }

    #[test]
    fn test_j() {
        assert_eq!(convert("й".to_owned()), "u");
    }

    #[test]
    fn test_k() {
        assert_eq!(convert("к".to_owned()), "k");
    }

    #[test]
    fn test_l() {
        assert_eq!(convert("л".to_owned()), "JI");
    }

    #[test]
    fn test_m() {
        assert_eq!(convert("м".to_owned()), "M");
    }

    #[test]
    fn test_n() {
        assert_eq!(convert("н".to_owned()), "H");
    }

    #[test]
    fn test_o() {
        assert_eq!(convert("о".to_owned()), "o");
    }

    #[test]
    fn test_p() {
        assert_eq!(convert("п".to_owned()), "n");
    }

    #[test]
    fn test_r() {
        assert_eq!(convert("р".to_owned()), "p");
    }

    #[test]
    fn test_s() {
        assert_eq!(convert("с".to_owned()), "c");
    }

    #[test]
    fn test_t() {
        assert_eq!(convert("т".to_owned()), "m");
    }

    #[test]
    fn test_u() {
        assert_eq!(convert("у".to_owned()), "y");
    }

    #[test]
    fn test_f() {
        assert_eq!(convert("ф".to_owned()), "qp");
    }

    #[test]
    fn test_x() {
        assert_eq!(convert("х".to_owned()), "x");
    }

    #[test]
    fn test_c() {
        assert_eq!(convert("ц".to_owned()), "u");
    }

    #[test]
    fn test_ch() {
        assert_eq!(convert("ч".to_owned()), "4");
    }

    #[test]
    fn test_sh() {
        assert_eq!(convert("ш".to_owned()), "LLI");
    }

    #[test]
    fn test_shh() {
        assert_eq!(convert("щ".to_owned()), "LLj");
    }

    #[test]

    fn test_tverdyi_znak() {
        assert_eq!(convert("ъ".to_owned()), "b");
    }

    #[test]
    fn test_yy() {
        assert_eq!(convert("ы".to_owned()), "bI");
    }

    #[test]
    fn test_myagkiy_znak() {
        assert_eq!(convert("ь".to_owned()), "b");
    }

    #[test]
    fn test_ee() {
        assert_eq!(convert("э".to_owned()), "3");
    }

    #[test]
    fn test_yu() {
        assert_eq!(convert("ю".to_owned()), "IO");
    }

    #[test]
    fn test_ya() {
        assert_eq!(convert("я".to_owned()), "R");
    }

    #[test]
    fn test_A() {
        assert_eq!(convert("А".to_owned()), "A");
    }

    #[test]
    fn test_B() {
        assert_eq!(convert("Б".to_owned()), "6");
    }

    #[test]
    fn test_V() {
        assert_eq!(convert("В".to_owned()), "B");
    }

    #[test]
    fn test_G() {
        assert_eq!(convert("Г".to_owned()), "r");
    }

    #[test]
    fn test_D() {
        assert_eq!(convert("Д".to_owned()), "D");
    }

    #[test]
    fn test_E() {
        assert_eq!(convert("Е".to_owned()), "E");
    }

    #[test]
    fn test_Yo() {
        assert_eq!(convert("Ё".to_owned()), "E");
    }

    #[test]
    fn test_Zh() {
        assert_eq!(convert("Ж".to_owned()), "ZH");
    }

    #[test]
    fn test_Z() {
        assert_eq!(convert("З".to_owned()), "3");
    }

    #[test]
    fn test_I() {
        assert_eq!(convert("И".to_owned()), "u");
    }

    #[test]
    fn test_J() {
        assert_eq!(convert("Й".to_owned()), "u");
    }

    #[test]
    fn test_K() {
        assert_eq!(convert("К".to_owned()), "K");
    }

    #[test]
    fn test_L() {
        assert_eq!(convert("Л".to_owned()), "JI");
    }

    #[test]
    fn test_M() {
        assert_eq!(convert("М".to_owned()), "M");
    }

    #[test]
    fn test_N() {
        assert_eq!(convert("Н".to_owned()), "H");
    }

    #[test]
    fn test_O() {
        assert_eq!(convert("О".to_owned()), "O");
    }

    #[test]
    fn test_P() {
        assert_eq!(convert("П".to_owned()), "n");
    }

    #[test]
    fn test_R() {
        assert_eq!(convert("Р".to_owned()), "P");
    }

    #[test]
    fn test_S() {
        assert_eq!(convert("С".to_owned()), "C");
    }

    #[test]
    fn test_T() {
        assert_eq!(convert("Т".to_owned()), "T");
    }

    #[test]
    fn test_U() {
        assert_eq!(convert("У".to_owned()), "Y");
    }

    #[test]
    fn test_F() {
        assert_eq!(convert("Ф".to_owned()), "qp");
    }

    #[test]
    fn test_X() {
        assert_eq!(convert("Х".to_owned()), "X");
    }

    #[test]
    fn test_C() {
        assert_eq!(convert("Ц".to_owned()), "U");
    }

    #[test]
    fn test_Ch() {
        assert_eq!(convert("Ч".to_owned()), "4");
    }

    #[test]
    fn test_Sh() {
        assert_eq!(convert("Ш".to_owned()), "LLI");
    }

    #[test]
    fn test_Shh() {
        assert_eq!(convert("Щ".to_owned()), "LLj");
    }

    #[test]
    fn test_Tverdiy_znak() {
        assert_eq!(convert("Ъ".to_owned()), "b");
    }

    #[test]
    fn test_YY() {
        assert_eq!(convert("Ы".to_owned()), "bI");
    }

    #[test]
    fn test_Myagkiy_znak() {
        assert_eq!(convert("Ь".to_owned()), "b");
    }

    #[test]
    fn test_Ee() {
        assert_eq!(convert("Э".to_owned()), "3");
    }

    #[test]
    fn test_Yu() {
        assert_eq!(convert("Ю".to_owned()), "IO");
    }

    #[test]
    fn test_Ya() {
        assert_eq!(convert("Я".to_owned()), "R");
    }
}
