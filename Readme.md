Library to help you profile your rust code. 

You wrap the work units in calls to the stack! macro. 
Entering a work unit "pushes" a context onto an explicit work stack (needed 
because LLVM's optimization eliminates much of the call stack), measuring 
how many nano-seconds you spend on the work unit, and pop it when done.
The stack helps understand nested work units.

For now output is send to stdout for manual interpretation, so stack 
carefully inside loops. Stacking inside a recursive function is not recommended.
