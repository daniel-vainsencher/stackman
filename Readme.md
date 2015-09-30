Library to help you profile your rust code.

You wrap the work units in calls to the stack! macro.
Entering a work unit "pushes" a context onto an explicit work stack (needed
because LLVM's optimization eliminates much of the call stack), measuring
how many nano-seconds you spend on the work unit, and pop it when done.
The stack keeps track of nested work units for us.

For now output is send to stdout for manual interpretation, so stack
carefully inside loops. Stacking inside a recursive function is not recommended.

Example:

        #[macro_use] extern crate stackman;

        fn algorithm() {
          stack!("PartA", {
            stack!("PartA1", {
                // Do stuff here
            });
            stack!("PartA2", {
              // and here
            });
          });
          stack!("PartB", {
              // Completely other stuff here
          });
        }

Running this writes output like

        Push PartA1
        Push PartA
        Pop PartA1 spent 6033107ns.
        Push PartA2
        Pop PartA2 spent 6033107ns.
        Pop PartA spent 16033107ns.
        Push PartB
        Pop PartB spent 1607ns.
