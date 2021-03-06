pub struct ExecutionOptions
{
  pub print_every_operation:bool
}


pub fn init_exec_options() -> ExecutionOptions
{
    ExecutionOptions
    {
        print_every_operation:false
    }
}