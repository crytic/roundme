use std::io::Error;

use crate::{
    ast::{Expr, Opcode},
    config, utils,
};

fn handle_pow(left: &Expr, config: &mut config::Config) -> Result<(), Error> {
    let expr_str = format!("{left}");

    if config
        .less_than_one
        .as_ref()
        .map_or(false, |vec| vec.contains(&expr_str))
    {
        return Ok(());
    }

    if config
        .greater_than_one
        .as_ref()
        .map_or(false, |vec| vec.contains(&expr_str))
    {
        return Ok(());
    }

    println!("Is {} greater than 1? Y/N (yes, no)", &left);

    if utils::ask_yes_no()? {
        config.add_greater_than_one(expr_str);
    } else {
        config.add_less_than_one(expr_str);
    };
    Ok(())
}

fn visit(expr: &Expr, config: &mut config::Config) {
    match expr {
        Expr::Number(_) | Expr::Id(_) | Expr::Error => (),
        Expr::Op(left, op, right) => {
            if let Opcode::Pow = op {
                // We ignore if the following fail
                let _ = handle_pow(left, config);
            }
            visit(left, config);
            visit(right, config);
        }
    };
}

pub fn find_less_greater_than_one(expr: &Expr, config: &mut config::Config) {
    visit(expr, config);
}
