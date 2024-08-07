use crate::actions::add_value::AddValueAction;
use crate::actions::all::CalculatorActions;
use crate::actions::multiply_by::MultiplyByAction;
use crate::solver::Solver;

#[test]
fn has_solution() {
    let mut solver = Solver {
        input: 10,
        output: 50,
        moves: 3,
        actions: vec![],
        portals: None,
    };

    solver.add_action(CalculatorActions::MultiplyBy(MultiplyByAction { value: 2 }));
    solver.add_action(CalculatorActions::MultiplyBy(MultiplyByAction { value: 3 }));
    solver.add_action(CalculatorActions::AddValue(AddValueAction { value: -5 }));

    let solution = solver.evaluate();
    assert!(solution.is_some());
    let solution = solution.unwrap();

    assert_eq!(
        solution,
        vec![vec![
            CalculatorActions::MultiplyBy(MultiplyByAction { value: 3 }),
            CalculatorActions::AddValue(AddValueAction { value: -5 }),
            CalculatorActions::MultiplyBy(MultiplyByAction { value: 2 }),
        ]]
    );
}

#[test]
fn correct_solution() {
    let mut solver = Solver {
        input: 10,
        output: 50,
        moves: 3,
        actions: vec![],
        portals: None,
    };

    solver.add_action(CalculatorActions::MultiplyBy(MultiplyByAction { value: 3 }));
    solver.add_action(CalculatorActions::AddValue(AddValueAction { value: -5 }));
    solver.add_action(CalculatorActions::MultiplyBy(MultiplyByAction { value: 2 }));

    let solution = &vec![
        &CalculatorActions::MultiplyBy(MultiplyByAction { value: 3 }),
        &CalculatorActions::AddValue(AddValueAction { value: -5 }),
        &CalculatorActions::MultiplyBy(MultiplyByAction { value: 2 }),
    ];

    let out = solver.evaluate_one_combination(solution);
    assert!(out.is_some());
}
