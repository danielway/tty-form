/*
Basic flow:
    - a "Form" is initialized via API
        - schema is defined via the API, e.g. TTYForm::new(schemaHere)
    - initialization also invokes the form to render
    - a loop begins
        - loop is reading stdin
    - input trickles-down
        - first level is application, for CTRL+C
        - second level is step, for ENTER/TAB to advance
            - if not last control, advance controls
            - if last control, emit to application that we need to advance steps
        - third level is control, for Char, LEFT/RIGHT/UP/DOWN
    - when complete (quit or advanced from last step) return the form data

Some notes around schema and data:
    - schema should be serializable
        - let the consumer load from file if they want
    - data should be created in a separate data structure alongside schema
        - it should omit any schema details and only contain the final product
        - it should target simple human readable display (.ToString?) but
            should be open to serialization in the future
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
