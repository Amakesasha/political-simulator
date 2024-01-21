pub mod buffer {
    use crate::*;

    #[test]
    fn buffer() {
        BufferS::test();
    }
}

pub mod gui {
    use crate::*;
    
    #[test]
    fn gui() {
        GuiS::test();
    }

    #[test]
    fn window() {
        WindowS::test();
    }

    #[test]
    fn button() {
        ButtonS::test();
    }

    #[test]
    fn table() {
        TableS::test();
    }
}