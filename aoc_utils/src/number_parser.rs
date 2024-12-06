///! Common task in AoC Input consist of numbers with fillertext. 
/// Sometimes fillertext is significant, but most times only integer numbers are  relevant (preserving the order)
/// parse_numbers<T>() -> Vec<T> where T:number (?) z.B. "7 bla blub 8 fasel" -> [7,8], "blah -3 xyz (3,4,5)" -> [-3, 3,4,5]
/// split_with_numbers -> Vec<?> z.B: "7 bla blub 8 fasel" -> [7, "bla blub", 8, "fasel"] in some form, maybe AST
/// unsigned variants where - is just a separator?
/// split variants where additional separators are used to split string?
/// 
/// There is no trait for number? num provides an PrimInt trait

// use num::Integer;

// pub fn parse_numbers<T>(s :&str) -> Vec<T>
//     where T: Integer
//     {

//     }