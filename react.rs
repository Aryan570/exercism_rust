#![allow(non_snake_case)]
use std::{collections::HashMap, fmt::Debug, hash::Hash};
/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq,Hash)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq,Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq,Hash)]
pub struct CallbackId(usize);

struct CellState<T>{
    value : T,
    to_update : Vec<ComputeCellId>
}
#[derive(Clone, Copy, Debug, PartialEq, Eq,Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}
struct ComputeCell<'a,T>{
    inputs : Vec<CellId>,
    compute_func : Box<dyn Fn(&[T]) -> T + 'a>,
    call_back_funcs : HashMap<CallbackId,Box<dyn FnMut(T) +'a>>

}
pub struct Reactor<'a,T> {
    cells : HashMap<CellId,CellState<T>>,
    compute_cell : HashMap<ComputeCellId,ComputeCell<'a,T>>,
    next : usize
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a,T: Copy + PartialEq + Debug> Reactor<'a,T> {
    pub fn new() -> Self {
        Reactor { cells: HashMap::new(), compute_cell: HashMap::new(), next: 0 }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let cell_id = InputCellId(self.next);
        self.next += 1;
        self.cells.insert(
            CellId::Input(cell_id),
            CellState { value: initial, to_update: Vec::new() }
        );
        cell_id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        for cell_id in dependencies{
            if !self.cells.contains_key(cell_id){
                return Err(*cell_id);
            }
        }
        let compute_cell_id = ComputeCellId(self.next);
        self.next += 1;
        let mut args = Vec::new();
        for cell_id in dependencies{
            let cell = self.cells.get_mut(cell_id).unwrap();
            cell.to_update.push(compute_cell_id);
            args.push(cell.value);
        }
        self.cells.insert(CellId::Compute(compute_cell_id), CellState { value: compute_func(&args), to_update: Vec::new() });
        self.compute_cell.insert(compute_cell_id, ComputeCell { inputs: dependencies.to_vec(), compute_func: Box::new(compute_func), call_back_funcs: HashMap::new() });
        Ok(compute_cell_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.cells.get(&id).map(|cell| cell.value)
    }

    // compute ids ---> value
    fn collect_outputs(&self, outputs: &mut HashMap<ComputeCellId, T>, compute_id: ComputeCellId) {
        if outputs.contains_key(&compute_id){return;}
        let cell = self.cells.get(&CellId::Compute(compute_id)).unwrap();
        outputs.insert(compute_id, cell.value);
        for &compute_id in &cell.to_update{self.collect_outputs(outputs, compute_id);}
    }
    fn recompute(&mut self,changed : &mut HashMap<ComputeCellId,T>,compute_id : ComputeCellId){
        let curr_val = changed.remove(&compute_id);
        if curr_val.is_none(){return;}
        let curr_val = curr_val.unwrap();
        let dependencies = self.compute_cell.get(&compute_id).unwrap().inputs.to_owned();
        let mut args = Vec::new();
        for depen_id in dependencies{
            if let CellId::Compute(dep_compute_id) = depen_id{self.recompute(changed, dep_compute_id);}
            args.push(self.cells.get(&depen_id).unwrap().value);
        }
        let compute_cell = self.compute_cell.get_mut(&compute_id).unwrap();
        let cell = self.cells.get_mut(&CellId::Compute(compute_id)).unwrap();
        cell.value = (compute_cell.compute_func)(&args);
        if cell.value != curr_val{
            for callback in compute_cell.call_back_funcs.values_mut(){
                callback(cell.value);
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let cell = self.cells.get_mut(&CellId::Input(id));
        if cell.is_none() {return false;}
        let cell = cell.unwrap();
        cell.value = new_value;
        let cell = self.cells.get(&CellId::Input(id)).unwrap();
        let mut outputs = HashMap::new();
        for compute_id in &cell.to_update{self.collect_outputs(&mut outputs, *compute_id);}
        while let Some(&compute_id) = outputs.keys().next(){
            self.recompute(&mut outputs, compute_id);
        }
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let cell = self.compute_cell.get_mut(&id)?;
        let call_back_id = CallbackId(self.next);
        self.next += 1;
        cell.call_back_funcs.insert(call_back_id, Box::new(callback));
        Some(call_back_id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let compute_cell = self.compute_cell.get_mut(&cell).ok_or(RemoveCallbackError::NonexistentCell)?;
        compute_cell.call_back_funcs.remove(&callback).ok_or(RemoveCallbackError::NonexistentCallback).map(|_| ())
    }
}
