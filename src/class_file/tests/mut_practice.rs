/// 参照と借用のため、次の構造体をバリデーションすることを考えます。
/// Elems は list の各要素のバリデーションが全て成功した時にバリデーション成功とします。
struct Elems {
    list: Vec<Elem>,
}

/// Elem のバリデーションは次の規則に従います。
/// - Atomic の場合、 value が Ok であればバリデーション成功、Err であればバリデーション失敗とします。
/// - Ref の場合、 index はこの構造体を含んでいる Elems の list の index 番地が有効なインデックスかつ
///     その番地のElemがバリデーションが成功した時のみ成功とします。
/// - Ref2 の場合、 index1, index2 の両方が Ref と同様の規則でバリデーションに成功した時のみ成功とします。
enum Elem {
    Atomic { value: Result<(), String> },
    Ref { index: usize },
    Ref2 {
        index1: usize,
        index2: usize,
    },
}

/// Ref は循環参照を起こす可能性がありますが、バリデーション中に無限ループを起こしてはいけません。
///
/// エラーの内容は適当でかまいません。
#[derive(Clone, PartialEq, Debug)]
enum Status {
    Done,
    Undone,
    Validating,
}

fn validate(elems: &Elems) -> Result<(), String> {
    let mut statuses = vec![Status::Undone; elems.list.len()];
    for index in 0..elems.list.len() {
        validate_each(index, elems, &mut statuses)?
    }
    Ok(())
}

fn validate_each(index: usize, elems: &Elems, statuses: &mut Vec<Status>) -> Result<(), String> {
    println!("index: {}, {:?}", index, statuses);


    let result = match elems.list.get(index) {
        None => Err(format!("invalid reference found! ref: {}", index)),
        Some(elem) => {
            if statuses[index] == Status::Done { return Ok(()); }

            statuses[index] = Status::Validating;
            match elem {
                Elem::Atomic { value } => value.to_owned(),
                Elem::Ref { index: ref_index } => {
                    if statuses[*ref_index] == Status::Validating {
                        return Err(format!("circular reference found! ref: {}", index));
                    }
                    validate_each(*ref_index, elems, statuses)
                }
                Elem::Ref2 { index1: ref_index1, index2: ref_index2 } => {
                    if statuses[*ref_index1] == Status::Validating {
                        return Err(format!("circular reference found! ref: {}", index));
                    }
                    validate_each(*ref_index1, elems, statuses)?;
                    if statuses[*ref_index2] == Status::Validating {
                        return Err(format!("circular reference found! ref: {}", index));
                    }
                    validate_each(*ref_index2, elems, statuses)?; // あれ？これ statuses の部分コンパイル通るの？
                    Ok(())
                }
            }
        }
    };
    if result.is_ok() {
        statuses[index] = Status::Done;
    }
    result
}

#[test]
fn test1() {
    let elems = Elems {
        list: vec![
            Elem::Atomic { value: Ok(()) },
            Elem::Ref { index: 0 },
        ]
    };
    assert_eq!(validate(&elems), Ok(()))
}

#[test]
fn test2() {
    let elems = Elems {
        list: vec![
            Elem::Ref { index: 0 }
        ]
    };
    assert_eq!(validate(&elems), Err("circular reference found! ref: 0".to_string()))
}

#[test]
fn test3() {
    let elems = Elems {
        list: vec![
            Elem::Ref { index: 1 },
            Elem::Ref { index: 0 },
        ]
    };
    assert_eq!(validate(&elems), Err("circular reference found! ref: 1".to_string()))
}

#[test]
fn test4() {
    let elems = Elems {
        list: vec![
            Elem::Atomic { value: Ok(()) },
            Elem::Atomic { value: Ok(()) },
            Elem::Ref2 { index1: 0, index2: 1 },
        ]
    };
    assert_eq!(validate(&elems), Ok(()))
}

#[test]
fn test5() {
    let elems = Elems {
        list: vec![
            Elem::Atomic { value: Ok(()) },
            Elem::Ref2 { index1: 0, index2: 1 },
        ]
    };
    assert_eq!(validate(&elems), Err("circular reference found! ref: 1".to_string()))
}
