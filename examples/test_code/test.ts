import { Something } from "some-module";

//##exemplify-start##{name = "multi-file-example" part=2}
function nestedExample() {

}
//##exemplify-end##

//##exemplify-start##{name="foo/bar" part=1}
export class Foobar {
    public doSomethingWorthwhile() {
        console.log("This is important!");
        return new Something();
    }
//##exemplify-end##
    private weDontCareAboutThis() {
        console.log("Not relevant...");
    }
//##exemplify-start##{name="foo/bar" part=2}
}
