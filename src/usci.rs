use std::str::FromStr;
use rand::rngs::ThreadRng;
use rand::Rng;
use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Code01 {
  C01,
  C02,
  C03,
  C04,
  C05,
  C06,
  C07,
  C08,
  C09,
  C0A,
  C0N,
  C0Y
}

impl Code01 {

  pub(crate) fn code_type() -> &'static str {
    return "登记管理部门";
  }

  pub(crate) fn get_code(&self) -> char {
    match self {
      Code01::C01 => '1',
      Code01::C02 => '2',
      Code01::C03 => '3',
      Code01::C04 => '4',
      Code01::C05 => '5',
      Code01::C06 => '6',
      Code01::C07 => '7',
      Code01::C08 => '8',
      Code01::C09 => '9',
      Code01::C0A => 'A',
      Code01::C0N => 'N',
      Code01::C0Y => 'Y',
    }
  }

  pub(crate) fn get_type(&self) -> &'static str {
    match self {
      Code01::C01 => "机构编制",
      Code01::C02 => "外交",
      Code01::C03 => "司法行政",
      Code01::C04 => "文化",
      Code01::C05 => "民政",
      Code01::C06 => "旅游",
      Code01::C07 => "宗教",
      Code01::C08 => "工会",
      Code01::C09 => "工商",
      Code01::C0A => "中央军委改革和编制办公室",
      Code01::C0N => "农业",
      Code01::C0Y => "其他",
    }
  }

  pub fn from_char(ch: char) -> Result<Self> {
    match ch {
     '1' => Ok(Code01::C01),
     '2' => Ok(Code01::C02),
     '3' => Ok(Code01::C03),
     '4' => Ok(Code01::C04),
     '5' => Ok(Code01::C05),
     '6' => Ok(Code01::C06),
     '7' => Ok(Code01::C07),
     '8' => Ok(Code01::C08),
     '9' => Ok(Code01::C09),
     'A' => Ok(Code01::C0A),
     'N' => Ok(Code01::C0N),
     'Y' => Ok(Code01::C0Y),
      _ => bail!(format!("Parse Code01 Error: {}", ch))
    }
  }

  pub fn from_random(rng: &mut ThreadRng) -> Self {
    let random = rng.gen::<usize>() % 12;
    match random {
      0 => Code01::C01,
      1 => Code01::C02,
      2 => Code01::C03,
      3 => Code01::C04,
      4 => Code01::C05,
      5 => Code01::C06,
      6 => Code01::C07,
      7 => Code01::C08,
      8 => Code01::C09,
      9 => Code01::C0A,
      10 => Code01::C0N,
      11 => Code01::C0Y,
      _ => unreachable!()
    }
  }

}

#[derive(Debug)]
pub enum Code02 {
  D11,
  D12,
  D13,
  D19,
  E21,
  E29,
  F31,
  F32,
  F33,
  F34,
  F35,
  F39,
  G41,
  G49,
  H51,
  H52,
  H53,
  H59,
  I61,
  I62,
  I69,
  J71,
  J72,
  J79,
  K81,
  K89,
  L91,
  L92,
  L93,
  MA1,
  MA9,
  NN1,
  NN2,
  NN3,
  NN9,
  P01
}

impl Code02 {

  pub(crate) fn code_type() -> &'static str {
    return "机构类别";
  }

  pub(crate) fn get_code(&self) -> char {
    match self {
      Code02::D11 => '1',
      Code02::D12 => '2',
      Code02::D13 => '3',
      Code02::D19 => '9',
      Code02::E21 => '1',
      Code02::E29 => '9',
      Code02::F31 => '1',
      Code02::F32 => '2',
      Code02::F33 => '3',
      Code02::F34 => '4',
      Code02::F35 => '5',
      Code02::F39 => '9',
      Code02::G41 => '1',
      Code02::G49 => '9',
      Code02::H51 => '1',
      Code02::H52 => '2',
      Code02::H53 => '3',
      Code02::H59 => '9',
      Code02::I61 => '1',
      Code02::I62 => '2',
      Code02::I69 => '9',
      Code02::J71 => '1',
      Code02::J72 => '2',
      Code02::J79 => '9',
      Code02::K81 => '1',
      Code02::K89 => '9',
      Code02::L91 => '1',
      Code02::L92 => '2',
      Code02::L93 => '9',
      Code02::MA1 => '1',
      Code02::MA9 => '9',
      Code02::NN1 => '1',
      Code02::NN2 => '2',
      Code02::NN3 => '3',
      Code02::NN9 => '9',
      Code02::P01 => '1'
    }
  }

  pub(crate) fn get_type(&self) -> &'static str {
    match self {
      Code02::D11 => "机关",
      Code02::D12 => "事业单位",
      Code02::D13 => "编办直接管理机构编制的群众团体",
      Code02::D19 => "其他",
      Code02::E21 => "外国常驻新闻机构",
      Code02::E29 => "其他",
      Code02::F31 => "律师执业机构",
      Code02::F32 => "公证处",
      Code02::F33 => "基层法律服务所",
      Code02::F34 => "司法鉴定机构",
      Code02::F35 => "仲裁委员会",
      Code02::F39 => "其他",
      Code02::G41 => "外国在华文化中心",
      Code02::G49 => "其他",
      Code02::H51 => "社会团体",
      Code02::H52 => "民办非企业单位",
      Code02::H53 => "基金会",
      Code02::H59 => "其他",
      Code02::I61 => "外国旅游部门常驻代表机构",
      Code02::I62 => "港澳台地区旅游部门常驻内地(大陆)代表机构",
      Code02::I69 => "其他",
      Code02::J71 => "宗教活动场所",
      Code02::J72 => "宗教院校",
      Code02::J79 => "其他",
      Code02::K81 => "基层工会",
      Code02::K89 => "其他",
      Code02::L91 => "企业",
      Code02::L92 => "个体工商户",
      Code02::L93 => "农民专业合作社",
      Code02::MA1 => "军队事业单位",
      Code02::MA9 => "其他",
      Code02::NN1 => "组级集体经济组织",
      Code02::NN2 => "村级集体经济组织",
      Code02::NN3 => "乡镇级集体经济组织",
      Code02::NN9 => "其他",
      Code02::P01 => ""
    }
  }

  pub fn from_char(code01: &Code01, ch: char) -> Result<Self> {
    match code01 {
      Code01::C01 => {
        match ch {
          '1' => Ok(Code02::D11),
          '2' => Ok(Code02::D12),
          '3' => Ok(Code02::D13),
          '9' => Ok(Code02::D19),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C02 => {
        match ch {
          '1' => Ok(Code02::E21),
          '2' => Ok(Code02::E29),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C03 => {
        match ch {
          '1' => Ok(Code02::F31),
          '2' => Ok(Code02::F32),
          '3' => Ok(Code02::F33),
          '4' => Ok(Code02::F34),
          '5' => Ok(Code02::F35),
          '9' => Ok(Code02::F39),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C04 => {
        match ch {
          '1' => Ok(Code02::G41),
          '9' => Ok(Code02::G49),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C05 => {
        match ch {
          '1' => Ok(Code02::H51),
          '2' => Ok(Code02::H52),
          '3' => Ok(Code02::H53),
          '9' => Ok(Code02::H59),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C06 => {
        match ch {
          '1' => Ok(Code02::I61),
          '2' => Ok(Code02::I62),
          '9' => Ok(Code02::I69),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C07 => {
        match ch {
          '1' => Ok(Code02::J71),
          '2' => Ok(Code02::J72),
          '9' => Ok(Code02::J79),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C08 => {
        match ch {
          '1' => Ok(Code02::K81),
          '9' => Ok(Code02::K89),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C09 => {
        match ch {
          '1' => Ok(Code02::L91),
          '2' => Ok(Code02::L92),
          '3' => Ok(Code02::L93),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C0A => {
        match ch {
          '1' => Ok(Code02::MA1),
          '9' => Ok(Code02::MA9),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C0N => {
        match ch {
          '1' => Ok(Code02::NN1),
          '2' => Ok(Code02::NN2),
          '3' => Ok(Code02::NN3),
          '9' => Ok(Code02::NN9),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
      Code01::C0Y => {
        match ch {
          '1' => Ok(Code02::P01),
          _ => bail!(format!("Parse Code02 Error (Code01: {}): {}", code01.get_code(), ch))
        }
      }
    }
  }

  pub fn from_random(code01: &Code01, rng: &mut ThreadRng) -> Self {
    let random = rng.gen::<usize>();
    match code01 {
      Code01::C01 => {
        let number = random % 4;
        match number {
          0 => Code02::D11,
          1 => Code02::D12,
          2 => Code02::D13,
          3 => Code02::D19,
          _ => unreachable!()
        }
      }
      Code01::C02 => {
        let number = random % 2;
        match number {
          0 => Code02::E21,
          1 => Code02::E29,
          _ => unreachable!()
        }
      }
      Code01::C03 => {
        let number = random % 6;
        match number {
          0 => Code02::F31,
          1 => Code02::F32,
          2 => Code02::F33,
          3 => Code02::F34,
          4 => Code02::F35,
          5 => Code02::F39,
          _ => unreachable!()
        }
      }
      Code01::C04 => {
        let number = random % 2;
        match number {
          0 => Code02::G41,
          1 => Code02::G49,
          _ => unreachable!()
        }
      }
      Code01::C05 => {
        let number = random % 4;
        match number {
          0 => Code02::H51,
          1 => Code02::H52,
          2 => Code02::H53,
          3 => Code02::H59,
          _ => unreachable!()
        }
      }
      Code01::C06 => {
        let number = random % 3;
        match number {
          0 => Code02::I61,
          1 => Code02::I62,
          2 => Code02::I69,
          _ => unreachable!()
        }
      }
      Code01::C07 => {
        let number = random % 3;
        match number {
          0 => Code02::J71,
          1 => Code02::J72,
          2 => Code02::J79,
          _ => unreachable!()
        }
      }
      Code01::C08 => {
        let number = random % 2;
        match number {
          0 => Code02::K81,
          1 => Code02::K89,
          _ => unreachable!()
        }
      }
      Code01::C09 => {
        let number = random % 3;
        match number {
          0 => Code02::L91,
          1 => Code02::L92,
          2 => Code02::L93,
          _ => unreachable!()
        }
      }
      Code01::C0A => {
        let number = random % 2;
        match number {
          0 => Code02::MA1,
          1 => Code02::MA9,
          _ => unreachable!()
        }
      }
      Code01::C0N => {
        let number = random % 4;
        match number {
          0 => Code02::NN1,
          1 => Code02::NN2,
          2 => Code02::NN3,
          3 => Code02::NN9,
          _ => unreachable!()
        }
      }
      Code01::C0Y => {
        Code02::P01
      }
    }
  }
}

const CODE03_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug)]
pub struct Code03 {
  code: String
}

impl Code03 {

  pub(crate) fn code_type() -> &'static str {
    return "登记管理机关行政区划码";
  }

  pub fn new(code: &str) -> Result<Code03> {
    if code.len() != 6 {
      bail!(format!("Parse Code03 Error: {}", code));
    }
    let mut code_chars = code.chars();
    while let Some(ch) = code_chars.next() {
      if !Code03::is_valid_char(ch) {
        bail!(format!("Parse Code03 Error: {}", code));
      }
    }
    return Ok(Code03 { code: code.to_string() });
  }

  pub fn is_valid_char(ch: char) -> bool {
    return CODE03_CHARS.contains(&ch);
  }

  pub fn from_random(rng: &mut ThreadRng) -> Self {
    let mut code03_str = String::with_capacity(6);
    for _ in 0..6 {
      let ch_index = rng.gen::<usize>() % 10;
      code03_str.push(CODE03_CHARS.get(ch_index).unwrap().clone());
    }
    return Code03 { code: code03_str };
  }

}

impl ToString for Code03 {
  fn to_string(&self) -> String {
    self.code.to_string()
  }
}

#[derive(Debug)]
pub struct Code04 {
  code: String
}

const CODE_CHARS: [char; 31] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'T', 'U', 'W', 'X', 'Y'];
const CODE_WEIGHT: [usize; 17] = [1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28];

impl Code04 {

  pub fn code_type() -> &'static str {
    return "组织机构代码";
  }

  pub fn new(code: &str) -> Result<Code04> {
    if code.len() != 9 {
      bail!(format!("Parse Code04 Error: {}", code));
    }
    let mut code_chars = code.chars();
    while let Some(ch) = code_chars.next() {
      if !Code04::is_valid_char(ch) {
        bail!(format!("Parse Code04 Error: {}", code));
      }
    }
    return Ok(Code04 { code: code.to_string() });
  }

  pub fn is_valid_char(ch: char) -> bool {
    return CODE_CHARS.contains(&ch);
  }

  pub fn from_random(rng: &mut ThreadRng) -> Self {
    let mut code04_str = String::with_capacity(9);
    for _ in 0..9 {
      let ch_index = rng.gen::<usize>() % 31;
      code04_str.push(CODE_CHARS.get(ch_index).unwrap().clone())
    }
    return Code04 { code: code04_str };
  }

}

impl ToString for Code04 {

  fn to_string(&self) -> String {
    self.code.to_string()
  }
}

#[derive(Debug)]
pub struct Code05 {
  code: char
}

impl Code05 {

  pub(crate) fn code_type() -> &'static str {
    return "校验码";
  }

}

impl ToString for Code05 {

  fn to_string(&self) -> String {
    self.code.to_string()

  }
}

#[derive(Debug)]
pub struct UsciCode {
  pub code01: Code01,
  pub code02: Code02,
  pub code03: Code03,
  pub code04: Code04,
  pub code05: Code05,
}

impl UsciCode {

  pub fn calculate_code05(code01: &Code01, code02: &Code02, code03: &Code03, code04: &Code04) -> Result<char> {
    let mut result = 0;
    result = result + UsciCode::calculate_char_weight(code01.get_code(), 0)?;
    result = result + UsciCode::calculate_char_weight(code02.get_code(), 1)?;
    let mut code3_chars = code03.code.chars();
    for index in 2..=7 {
      result = result + UsciCode::calculate_char_weight(code3_chars.next().unwrap(), index)?;
    }
    let mut code4_chars = code04.code.chars();
    for index in 8..=16 {
      result = result + UsciCode::calculate_char_weight(code4_chars.next().unwrap(), index)?;
    }
    result = result % 31;
    result = if result == 0 { 0 } else { 31 - result };
    Ok(CODE_CHARS.get(result).unwrap().clone())
  }

  fn calculate_char_weight(ch: char, index: usize) -> Result<usize> {
    match CODE_CHARS.binary_search(&ch) {
      Ok(position) => {
        Ok(CODE_WEIGHT.get(index).unwrap() * position)
      }
      Err(_) => {
        bail!(format!("Calculate Weight Error: {} {}", index, ch))
      }
    }
  }

  pub fn from_random(rng: &mut ThreadRng) -> Self {
    let code01 = Code01::from_random(rng);
    let code02 = Code02::from_random(&code01, rng);
    let code03 = Code03::from_random(rng);
    let code04 = Code04::from_random(rng);
    let code05_ch = UsciCode::calculate_code05(&code01, &code02, &code03, &code04).unwrap();
    return UsciCode {
      code01,
      code02,
      code03,
      code04,
      code05: Code05 { code: code05_ch }
    };
  }

  pub fn get_code(&self) -> String {
    let mut code_str = String::with_capacity(18);
    code_str.push(self.code01.get_code());
    code_str.push(self.code02.get_code());
    code_str.push_str(self.code03.code.as_str());
    code_str.push_str(self.code04.code.as_str());
    code_str.push(self.code05.code);
    return code_str;
  }
}

impl FromStr for UsciCode {

  type Err = anyhow::Error;

  fn from_str(code_str: &str) -> Result<Self> {
    if code_str.len() != 18 {
      bail!(format!("Parse Code Error: {}", code_str));
    }
    let mut code_chars = code_str.chars();
    let code01 = Code01::from_char(code_chars.next().unwrap())?;
    let code02 = Code02::from_char(&code01, code_chars.next().unwrap())?;
    let mut code03_str = String::with_capacity(6);
    for _ in 0..6 {
      code03_str.push(code_chars.next().unwrap());
    }
    let code03 = Code03::new(code03_str.as_str())?;
    let mut code04_str = String::with_capacity(9);
    for _ in 0..9 {
      code04_str.push(code_chars.next().unwrap());
    }
    let code04 = Code04::new(code04_str.as_str())?;
    let code05_ch = UsciCode::calculate_code05(&code01, &code02, &code03, &code04)?;
    if code05_ch != code_chars.next().unwrap() {
      bail!(format!("Verify Code05 Error: {}", code05_ch));
    }
    return Ok(UsciCode {
      code01,
      code02,
      code03,
      code04,
      code05: Code05 { code: code05_ch }
    });
  }
}