#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Requirement {
    #[default]
    Unused,
    Zero,
    One,
}

impl Requirement {
    pub(crate) fn from_bit(bit: bool) -> Self {
        if bit { Self::One } else { Self::Zero }
    }

    pub(crate) fn accepts(self, bit: bool) -> bool {
        matches!(self, Self::Unused | Self::Zero if !bit)
            || matches!(self, Self::Unused | Self::One if bit)
    }
}

/// A compositional output bit. A forwarded bit denotes a projection from one
/// of the two input channels instead of a materialized value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum QuadValue {
    #[default]
    Zero,
    One,
    FromParent(usize),
    FromSibling(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Decision {
    source: QuadValue,
    expected: bool,
}

pub(crate) fn materialize_quad(
    values: &[QuadValue],
    parent: &[bool],
    sibling: &[bool],
) -> Vec<bool> {
    values
        .iter()
        .copied()
        .map(|value| materialize_quad_value(value, parent, sibling))
        .collect()
}

pub(crate) fn materialize_quad_value(value: QuadValue, parent: &[bool], sibling: &[bool]) -> bool {
    match value {
        QuadValue::Zero => false,
        QuadValue::One => true,
        QuadValue::FromParent(index) => parent[index],
        QuadValue::FromSibling(index) => sibling[index],
    }
}

fn decision_for(value: QuadValue, parent: &[bool], sibling: &[bool]) -> Option<Decision> {
    match value {
        QuadValue::FromParent(index) => Some(Decision {
            source: value,
            expected: parent[index],
        }),
        QuadValue::FromSibling(index) => Some(Decision {
            source: value,
            expected: sibling[index],
        }),
        QuadValue::Zero | QuadValue::One => None,
    }
}

/// Specialize `raw || carry` using the current inputs. The returned decision
/// is the weakest condition needed for the chosen branch to remain valid.
pub(crate) fn specialize_or(
    raw: QuadValue,
    carry: QuadValue,
    parent: &[bool],
    sibling: &[bool],
) -> (QuadValue, Option<Decision>) {
    if materialize_quad_value(raw, parent, sibling) {
        (raw, decision_for(raw, parent, sibling))
    } else {
        (carry, decision_for(raw, parent, sibling))
    }
}

/// Accept states are observable and therefore must be concrete. The decision
/// records when that concrete result may safely be reused.
pub(crate) fn specialize_concrete(
    value: QuadValue,
    parent: &[bool],
    sibling: &[bool],
) -> (QuadValue, Option<Decision>) {
    let materialized = materialize_quad_value(value, parent, sibling);
    (
        if materialized {
            QuadValue::One
        } else {
            QuadValue::Zero
        },
        decision_for(value, parent, sibling),
    )
}

pub(crate) fn record_decision(
    decision: Option<Decision>,
    require_parent: &mut [Requirement],
    require_sibling: &mut [Requirement],
) {
    let Some(decision) = decision else { return };
    let requirement = Requirement::from_bit(decision.expected);
    match decision.source {
        QuadValue::FromParent(index) => require_parent[index] = requirement,
        QuadValue::FromSibling(index) => require_sibling[index] = requirement,
        QuadValue::Zero | QuadValue::One => unreachable!("constants do not create decisions"),
    }
}

#[cfg(kani)]
mod verification {
    use super::*;

    fn symbolic_value(tag: u8, index: usize) -> QuadValue {
        match tag {
            0 => QuadValue::Zero,
            1 => QuadValue::One,
            2 => QuadValue::FromParent(index),
            3 => QuadValue::FromSibling(index),
            _ => unreachable!(),
        }
    }

    fn decision_holds(decision: Option<Decision>, parent: &[bool], sibling: &[bool]) -> bool {
        decision.is_none_or(|decision| {
            materialize_quad_value(decision.source, parent, sibling) == decision.expected
        })
    }

    #[kani::proof]
    fn tri_requirement_reuse_is_sound() {
        let old: bool = kani::any();
        let new: bool = kani::any();
        let requirement = Requirement::from_bit(old);
        kani::assume(requirement.accepts(new));
        assert_eq!(new, old);
    }

    #[kani::proof]
    fn quad_projection_materializes_current_input() {
        let parent: [bool; 2] = kani::any();
        let sibling: [bool; 2] = kani::any();
        let index: usize = kani::any();
        kani::assume(index < 2);
        assert_eq!(
            materialize_quad_value(QuadValue::FromParent(index), &parent, &sibling),
            parent[index]
        );
        assert_eq!(
            materialize_quad_value(QuadValue::FromSibling(index), &parent, &sibling),
            sibling[index]
        );
    }

    #[kani::proof]
    fn specialized_or_refines_boolean_or() {
        let old_parent: [bool; 2] = kani::any();
        let old_sibling: [bool; 2] = kani::any();
        let new_parent: [bool; 2] = kani::any();
        let new_sibling: [bool; 2] = kani::any();
        let raw_tag: u8 = kani::any();
        let carry_tag: u8 = kani::any();
        let raw_index: usize = kani::any();
        let carry_index: usize = kani::any();
        kani::assume(raw_tag < 4 && carry_tag < 4);
        kani::assume(raw_index < 2 && carry_index < 2);
        let raw = symbolic_value(raw_tag, raw_index);
        let carry = symbolic_value(carry_tag, carry_index);
        let (specialized, decision) = specialize_or(raw, carry, &old_parent, &old_sibling);
        kani::assume(decision_holds(decision, &new_parent, &new_sibling));
        assert_eq!(
            materialize_quad_value(specialized, &new_parent, &new_sibling),
            materialize_quad_value(raw, &new_parent, &new_sibling)
                || materialize_quad_value(carry, &new_parent, &new_sibling)
        );
    }

    #[kani::proof]
    fn concrete_accept_reuse_is_sound() {
        let old_parent: [bool; 2] = kani::any();
        let old_sibling: [bool; 2] = kani::any();
        let new_parent: [bool; 2] = kani::any();
        let new_sibling: [bool; 2] = kani::any();
        let tag: u8 = kani::any();
        let index: usize = kani::any();
        kani::assume(tag < 4 && index < 2);
        let value = symbolic_value(tag, index);
        let (concrete, decision) = specialize_concrete(value, &old_parent, &old_sibling);
        kani::assume(decision_holds(decision, &new_parent, &new_sibling));
        assert_eq!(
            materialize_quad_value(concrete, &new_parent, &new_sibling),
            materialize_quad_value(value, &new_parent, &new_sibling)
        );
    }
}
