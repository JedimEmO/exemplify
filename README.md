# Exemplify - example code extractor for authors

## Purpose

I find it easier to keep example code green and alive if it is part of a compilable code base.
This tool lets you annotate sections of your source code, and have generated includeable sample files with the highlighted parts.

## Usage

First, annotate the example in your source file.
You can combine multiple blocks, they will appear ordered by the optional part parameter.


----
// This chunk will be indented by 4 spaces relative to the other chunks in this example
//##exemplify-start##{name="name of the example" part=1 indentation=4}
export class Foobar {
    doSomething() {}
}
//##exemplify-end##

function doNotShowThis() {
    // This will not be part of the example output file
}

//##exemplify-start##{name="name of the example" part=2}
new Foobar();
//##exemplify-end##
----

----
exemplify -s /path/to/example/code/root -o /path/to/output/folder -e "ts"
----

This should create the file *name of the example.adoc* in your output folder, with the following content:

----
    export class Foobar {
        doSomething() {}
    }
new Foobar();
----
