use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
     BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}
pub fn eval(expr: Expr) -> Value {
    match expr{
        ArithExpr(expr) => IntValue(eval_arith_expr(expr)),
        BoolExpr(expr) => BoolValue(eval_bool_expr(expr)),

    }

}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr{
        BinArithExpr { left, right, op } => {
            match op {
                AddOp => eval_arith_expr(*left) + eval_arith_expr(*right),
                SubOp => eval_arith_expr(*left) - eval_arith_expr(*right),
                MulOp => eval_arith_expr(*left) * eval_arith_expr(*right),
                IntDivOp => eval_arith_expr(*left) / eval_arith_expr(*right),
            }

        }
        IntLit(num) => num,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {
        ArithCmpExpr { left, right, op } =>{
            match op{
            LtOp => eval_arith_expr(*left) < eval_arith_expr(*right),
            LteOp => eval_arith_expr(*left) <= eval_arith_expr(*right),
            GtOp => eval_arith_expr(*left) > eval_arith_expr(*right),
            GteOp => eval_arith_expr(*left) >= eval_arith_expr(*right),
            ArithEqOp => eval_arith_expr(*left) == eval_arith_expr(*right),
            ArithNeqOp => eval_arith_expr(*left) != eval_arith_expr(*right),
            }

        }
        BinBoolExpr { left, right, op } =>{
            match op {
                AndOp => eval_bool_expr(*left) && eval_bool_expr(*right),
                OrOp => eval_bool_expr(*left) || eval_bool_expr(*right),
                BoolEqOp => eval_bool_expr(*left) == eval_bool_expr(*right),
                BoolNeqOp => eval_bool_expr(*left) != eval_bool_expr(*right),
            } 
        }
        BoolLit(boolean) => boolean,
        NotExpr(expr) => !eval_bool_expr(*expr),
    

    }

}



fn main() {}

mod tests {
    use super::*;

    #[test]
    fn test_arith1() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(69)), right: Box::new(IntLit(3)), op: AddOp } );
        let answer = IntValue(72);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_arith2() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(33)), right: Box::new(IntLit(16)), op: SubOp } );
        let answer = IntValue(17);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_arith3() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(9)), right: Box::new(IntLit(8)), op: MulOp } );
        let answer = IntValue(72);
        assert_eq!(eval(expr), answer);  
    }

    #[test]
    fn test_arith4() {
        let expr = ArithExpr(BinArithExpr { left: Box::new(IntLit(51)), right: Box::new(IntLit(17)), op: IntDivOp } );
        let answer = IntValue(3);
        assert_eq!(eval(expr), answer);  
    }
    

    #[test]
    fn test_arith5() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(33)), right: Box::new(IntLit(100)), op: (LtOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_arith6() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(88)), right: Box::new(IntLit(88)), op: (LteOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_arithcmpr3() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(33)), right: Box::new(IntLit(99)), op: (GtOp) });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_arithcmp4() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(88)), right: Box::new(IntLit(88)), op: (GteOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_arithcmp5() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(33)), right: Box::new(IntLit(33)), op: (ArithEqOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_arithcmp6() {
        let expr = BoolExpr(ArithCmpExpr { left: Box::new(IntLit(100)), right: Box::new(IntLit(69)), op: (ArithNeqOp) });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_bool1() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(true)), op: AndOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_bool2() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(false)), op: OrOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_bin_bool() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: OrOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_bin_bool3() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(false)), op: BoolEqOp });
        let answer = BoolValue(true);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_bin_bool4() {
        let expr = BoolExpr(BinBoolExpr { left: Box::new(BoolLit(false)), right: Box::new(BoolLit(false)), op: BoolNeqOp });
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }
    #[test]
    fn test_bin_bool5() {
        let expr = BoolExpr(NotExpr(Box::new(BoolLit(true))));
        let answer = BoolValue(false);
        assert_eq!(eval(expr), answer);
    }
    
    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}