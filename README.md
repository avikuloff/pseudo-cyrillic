# pseudo-cyrillic
Faux Russian typography / Фальшивая русская типографика

Эта маленькая библиотека содержит всего одну функцию, 
которая преобразует буквы русского алфавита в строке, 
в похожие по начертанию символы латиницы и цифры.

## Examples
```rust
assert_eq!(convert("Дровосек".to_owned()), "DpoBocek");
assert_eq!(convert("Воин Света".to_owned()), "BouH CBema");
assert_eq!(convert("Князь Тьмы".to_owned()), "KHR3b TbMbI");
```

## Таблица соответствия
Оригинал|Аналог на латинице|Оригинал|Аналог на латинице
:---:|:---:|:---:|:---:
а|a|А|A
б|6|Б|6
в|B|В|B
г|r|Г|r
д|D|Д|D
е|e|Е|E
ё|e|Ё|E
ж|zh|Ж|ZH
з|3|З|3
и|u|И|u
й|u|Й|u
к|k|К|K
л|JI|Л|JI
м|M|М|M
н|H|Н|H
о|o|О|O
п|n|П|n
р|p|Р|P
с|c|С|C
т|m|Т|T
у|y|У|Y
ф|qp|Ф|qp
х|x|Х|X
ц|u|Ц|U
ч|4|Ч|4
ш|LLI|Ш|LLI
щ|LLj|Щ|LLj
ъ|b|Ъ|b
ы|bI|Ы|bI
ь|b|Ь|b
э|3|Э|3
ю|IO|Ю|IO
я|R|Я|R


## License
`pseudo-cyrillic` is distributed under the terms of both the [MIT license](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).