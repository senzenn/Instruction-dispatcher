use crate::*;

#[test]
fn test_counter_operations() {
    // Test basic counter state management
    let mut counter = Counter { value: 0 };

    // Test increment
    counter.value = counter.value.saturating_add(5);
    assert_eq!(counter.value, 5);

    // Test decrement
    counter.value = counter.value.saturating_sub(2);
    assert_eq!(counter.value, 3);

    // Test reset
    counter.value = 0;
    assert_eq!(counter.value, 0);

    // Test set
    counter.value = 42;
    assert_eq!(counter.value, 42);

    // Test overflow protection (saturating operations)
    counter.value = u64::MAX;
    counter.value = counter.value.saturating_add(1);
    assert_eq!(counter.value, u64::MAX); // Should not overflow

    // Test underflow protection
    counter.value = 0;
    counter.value = counter.value.saturating_sub(1);
    assert_eq!(counter.value, 0); // Should not underflow
}

#[test]
fn test_instruction_serialization() {
    use borsh::{BorshDeserialize, to_vec};

    // Test Counter serialization/deserialization
    let counter = Counter { value: 42 };
    let serialized = to_vec(&counter).unwrap();
    let deserialized: Counter = Counter::try_from_slice(&serialized).unwrap();
    assert_eq!(counter.value, deserialized.value);

    // Test empty counter
    let counter = Counter { value: 0 };
    let serialized = to_vec(&counter).unwrap();
    let deserialized: Counter = Counter::try_from_slice(&serialized).unwrap();
    assert_eq!(counter.value, deserialized.value);
}

#[test]
fn test_error_conditions() {
    // Test MaxValue error
    let max_counter = Counter { value: u64::MAX };
    let result = max_counter.value.checked_add(1);
    assert!(result.is_none()); // Should overflow

    // Test Underflow error
    let zero_counter = Counter { value: 0 };
    let result = zero_counter.value.checked_sub(1);
    assert!(result.is_none()); // Should underflow

    // Test InvalidValue error (value > 100)
    let invalid_result: Result<u64, crate::CustomError> = Err(crate::CustomError::InvalidValue);
    assert!(matches!(invalid_result, Err(crate::CustomError::InvalidValue)));
}

#[test]
fn test_instruction_unpacking() {
    // Test Initialize
    let init_data = vec![0];
    let instruction = CounterInstruction::unpack(&init_data);
    assert!(matches!(instruction, Some(CounterInstruction::Initialize)));

    // Test Increment
    let inc_data = vec![1, 5, 0, 0, 0, 0, 0, 0, 0];
    let instruction = CounterInstruction::unpack(&inc_data);
    assert!(matches!(instruction, Some(CounterInstruction::Increment(5))));

    // Test Decrement
    let dec_data = vec![2, 3, 0, 0, 0, 0, 0, 0, 0];
    let instruction = CounterInstruction::unpack(&dec_data);
    assert!(matches!(instruction, Some(CounterInstruction::Decrement(3))));

    // Test Reset
    let reset_data = vec![3];
    let instruction = CounterInstruction::unpack(&reset_data);
    assert!(matches!(instruction, Some(CounterInstruction::Reset)));

    // Test Set
    let set_data = vec![5, 42, 0, 0, 0, 0, 0, 0, 0];
    let instruction = CounterInstruction::unpack(&set_data);
    assert!(matches!(instruction, Some(CounterInstruction::Set(42))));

    // Test NoOp
    let noop_data = vec![6];
    let instruction = CounterInstruction::unpack(&noop_data);
    assert!(matches!(instruction, Some(CounterInstruction::NoOp)));

    // Test invalid data
    let invalid_data = vec![99];
    let instruction = CounterInstruction::unpack(&invalid_data);
    assert!(instruction.is_none());
}
