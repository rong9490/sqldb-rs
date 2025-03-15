/* 词法分析 Lexer 定义 */

use super::token::Token;

// 支持的sql语法: 1.CreateTable 2.InsertInto 3.SelectFrom
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

// 实现方法
impl<'a> Lexer<'a> {
    // 实例化
    pub fn new(sql_text: &'a str) -> Self {
        Self {
            iter: sql_text.chars().peekable(),
        }
    }

    fn erase_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace()); // 传入处理的回调函数
    }

    // 如果满足条件，则跳转到下一个字符，并返回该字符
    fn next_if<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<char> {
        self.iter.peek().filter(|&c| predicate(*c))?; // 是否满足断言
        self.iter.next() // 是的话跳到下一个
    }

    // 辅助方法(类似于回调函数, 满足条件的断言函数), 满足则跳到下一个
    fn next_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> Option<String> {
        let mut value: String = String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c); // 一直有效一直存入
        }
        Some(value).filter(|v| !v.is_empty()) // 如果为空字符串转为返回None
    }

    // 扫描拿到下一个Token
    fn scan(&mut self) -> Result<Option<Token>> {
        // 清除字符串中空白字符部分
        self.erase_whitespace();

        // 根据第一个字符判断, 由X开头的
        match self.iter.peek() {
            Some('\'') => self.scan_string(), // 扫描字符串
            Some(c) if c.is_ascii_digit() => Ok(self.scan_number()), // 扫描数字
            Some(c) if c.is_alphabetic() => Ok(self.scan_ident()), // 扫描 Ident 类型
            Some(_) => Ok(self.scan_symbol()), // 扫描符号
            None => Ok(None),
        }
    }

    fn scan_string(&mut self) -> Result<Option<Token>> {
        // 判断是否单引号开头
        let mut value = String::new();
        while let Some(c) = self.next_if(|c| c != '\'') {
            value.push(c);
        }
    }
}

// 自定义迭代器, 返回Token
// impl<'a> Iterator for Lexer<'a> {

// }
