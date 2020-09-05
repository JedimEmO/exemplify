import { Something } from "some-module";

//##exemplify-start##{name = "multi-file-example" part=1 indentation=0}
    function nestedExample() {

    }
//##exemplify-end##

//##exemplify-start##{name="foo/bar" title="Test example 1" part=1}
export class Foobar {
    public doSomethingWorthwhile() {
        console.log("This is important!"); // ##callout##{value="this is a callout"}
        return new Something();
    }
//##exemplify-end##
    private weDontCareAboutThis() {
        console.log("Not relevant...");
    }
//##exemplify-start##{name="foo/bar" part=2 language="javascript"}
}
