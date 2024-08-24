use std::fmt::Display;

use enum_iterator::{all, Sequence};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uiua::{Primitive, SysOp, EXAMPLE_UA};

use crate::{editor::*, other_tutorial::OtherTutorialParams, title_markdown, Hd, Prim, Prims};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum TutorialPage {
    Introduction,
    Basic,
    Math,
    Arrays,
    Types,
    Bindings,
    Functions,
    AdvancedStack,
    Inverses,
    ControlFlow,
    PatternMatching,
    AdvancedArray,
    ThinkingWithArrays,
    Macros,
    TacitCode,
    Modules,
    Testing,
}

impl TutorialPage {
    pub fn path(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
    pub fn title(&self) -> &'static str {
        match self {
            Self::Introduction => "Introduction",
            Self::Basic => "Basic Stack Operations and Formatting",
            Self::Math => "Math and Comparison",
            Self::Arrays => "Arrays",
            Self::Types => "Types",
            Self::Bindings => "Bindings",
            Self::Functions => "Modifiers and Functions",
            Self::AdvancedStack => "Advanced Stack Manipulation",
            Self::Inverses => "Inverses",
            Self::ControlFlow => "Control Flow",
            Self::PatternMatching => "Pattern Matching",
            Self::AdvancedArray => "Advanced Array Manipulation",
            Self::ThinkingWithArrays => "Thinking With Arrays",
            Self::Macros => "Macros",
            Self::TacitCode => "Tacit Code",
            Self::Modules => "Modules",
            Self::Testing => "Testing",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Params)]
pub struct TutorialParams {
    page: TutorialPage,
}

#[component]
pub fn Tutorial() -> impl IntoView {
    move || match use_params::<TutorialParams>().get() {
        Ok(params) => {
            let page = params.page;
            let tut_view = match page {
                TutorialPage::Introduction => TutorialIntroduction().into_view(),
                TutorialPage::Basic => TutorialBasic().into_view(),
                TutorialPage::Math => TutorialMath().into_view(),
                TutorialPage::Arrays => TutorialArrays().into_view(),
                TutorialPage::Types => TutorialTypes().into_view(),
                TutorialPage::Bindings => TutorialBindings().into_view(),
                TutorialPage::Functions => TutorialFunctions().into_view(),
                TutorialPage::ControlFlow => TutorialControlFlow().into_view(),
                TutorialPage::AdvancedStack => TutorialAdvancedStack().into_view(),
                TutorialPage::Inverses => TutorialInverses().into_view(),
                TutorialPage::PatternMatching => TutorialPatternMatching().into_view(),
                TutorialPage::AdvancedArray => TutorialAdvancedArray().into_view(),
                TutorialPage::ThinkingWithArrays => TutorialThinkingWithArrays().into_view(),
                TutorialPage::Macros => TutorialMacros().into_view(),
                TutorialPage::TacitCode => TutorialTacitCode().into_view(),
                TutorialPage::Modules => TutorialModules().into_view(),
                TutorialPage::Testing => TutorialTesting().into_view(),
            };
            view! {
                <A href="/docs">"Back to Docs Home"</A>
                <br/>
                <br/>
                <TutorialNav page=page/>
                { tut_view }
                <br/>
                <br/>
                <TutorialNav page=page/>
                <br/>
                <br/>
                <A href="/docs">"Back to Docs Home"</A>
            }
            .into_view()
        }
        Err(_) => match use_params::<OtherTutorialParams>().get() {
            Ok(params) => view! {
                <A href="/docs">"Back to Docs Home"</A>
                <br/>
                <br/>
                { params.page.view() }
                <br/>
                <br/>
                <A href="/docs">"Back to Docs Home"</A>
            }
            .into_view(),
            Err(_) => TutorialIntroduction().into_view(),
        },
    }
}

impl IntoParam for TutorialPage {
    fn into_param(value: Option<&str>, name: &str) -> Result<Self, ParamsError> {
        if value == Some("custommodifiers") {
            return Ok(TutorialPage::Macros);
        }
        all::<TutorialPage>()
            .find(|p| p.path() == value.unwrap_or(""))
            .ok_or_else(|| ParamsError::MissingParam(name.to_string()))
    }
}

#[component]
fn TutorialNav(page: TutorialPage) -> impl IntoView {
    let next = move || {
        page.next()
            .map(|p| {
                view!( <div>"Next: "<A href=format!("/tutorial/{}", p.path())>{p.title()}</A>" 〉"</div>)
                    .into_view()
            })
            .unwrap_or_else(|| view!( <div/>).into_view())
    };
    let previous = move || {
        page.previous()
            .map(|p| {
                view!( <div>"〈 Previous: "<A href=format!("/tutorial/{}", p.path())>{p.title()}</A></div>)
                    .into_view()
            })
            .unwrap_or_else(|| view!( <div/>).into_view())
    };

    view! {
        <div class="tutorial-nav">
            { previous }
            { next }
        </div>
    }
}

#[component]
fn TutorialIntroduction() -> impl IntoView {
    view! {
        <Title text="Introduction - Uiua Docs"/>
        <h1>"Introduction"</h1>
        <p>"Welcome to the Uiua tutorial! Each page of this tutorial will introduce you to a new concept in the language. The tutorial is designed to be read in order, but you can jump around if you want."</p>
        <p>"Uiua is an array programming language in the same family as APL, J, and BQN. Uiua focuses particularly on tacit programming, that is, writing code without naming variables. It does this by putting all values on a global stack."</p>

        <Hd id="what-is-array-programming">"What is Array Programming?"</Hd>
        <p>"Before jumping into Uiua, it may be helpful to understand the paradigm of which it is a part."</p>
        <p><a href="https://en.wikipedia.org/wiki/Array_programming">"Array Programming"</a>" is a way of writing programs first developed by Ken Iverson in the 1960s. It emphasizes the use of arrays as the primary data structure, and many operations work on entire arrays at once. An array language also has a rich set of operations for manipulating arrays in various ways, to the point where many data structures and control flow constructs are replaced with operations on arrays."</p>

        <Hd id="why-array-programming">"Why Array Programming?"</Hd>
        <p>"There are many reasons that Array Programming is cool, but here are a few:"</p>
        <p>"Perhaps the most practical reason is that arrays as a data structure are what some refer to as \"machine-sympathetic\". Because array elements exist in contiguous memory, they can be processed very quickly by modern CPUs. Many operations can be "<em>"vectorized"</em>", meaning that the CPU can apply the operation to many elements in parallel. In addition, allocating memory for a large array can be done all at once. Although most array languages are dynamic and interpreted, they can often compete in performance with compiled, static languages, at least for certain kinds of tasks."</p>
        <p>"Array programming can change the way you think about programs. All operations you use act on a single, unified data structure. You stop thinking about individual items and start thinking about entire collections and how they can interact and transform."</p>
        <p>"Finally, many common algorithms are built-in to array languages, and are often written with a single glyph. This makes array code very concise and expressive. Several lines of code in a language like C or Python can often be just a few characters in an array language. Array languages do away with much of the "<em>"ceremony"</em>" that is present in other languages, so you can focus more on the problem you are trying to solve."</p>

        <Hd id="why-uiua">"Why Uiua?"</Hd>
        <p>"Uiua retains many of the array operations and semantics of its predecessor array languages. However, while APL, J, and BQN structure code with syntax and semantics based on mathematical notation:"</p>
        <p><code>"1 + 2"</code>", "<code>"4×ω-2"</code>", etc."</p>
        <p>"Uiua puts values on a stack, and operators appear to the left of their arguments:"</p>
        <p>""<code>"+ 1 2"</code>", "<code>"×4-2"</code>", etc."</p>
        <p>"This allows Uiua to be, in most cases, even more terse than other array languages. That being said, Uiua has been designed to remain readable as much as possible, even without named local variables. While writing tacit code can quickly become unwieldy in other array languages, Uiua embraces tacitness as the only way."</p>
        <p>"Uiua also features built-in functionality for working with images, audio, and GIFs, so once you learn the language, you can very quickly get started writing programs that do interesting things!"</p>
        <p>"Note that Uiua is not yet stable. While most of the core features are unlikely to change much, more complex ones are still in flux. This tutorial is based on the current state of the language, and it is updated as the language changes."</p>
        <p><strong>"Important Note"</strong>": This website usually uses the most recent development version of Uiua, rather than the stable release. This is so people can test new features easily. Some features available here may not be available if you install Uiua locally in the default way. The version of Uiua currently running on this site is "<code>{uiua::VERSION}</code>"."</p>

        <Hd id="who">"Who is this tutorial for?"</Hd>
        <p>"This tutorial is targeted at people who have at least a little bit of experience with programming. While you don't need to be proficient in any particular language, it will be helpful to understand concepts like variables and functions. However, even if you are a beginner, you can likely find your way through by taking the time to understand each example."</p>
        <p>"Click the link to the next section to get started!"</p>
    }
}

#[component]
fn TutorialBasic() -> impl IntoView {
    use Primitive::*;

    let primitive_table: Vec<_> = Primitive::all()
        .filter_map(|p| {
            if let (Some(ascii), Some(unicode)) = (p.ascii(), p.glyph()) {
                if ascii.to_string() != unicode.to_string() {
                    return Some(view! {
                        <tr>
                            <td><code>{ p.name() }</code></td>
                            <td><code>{ ascii.to_string() }</code></td>
                            <td><Prim prim=p glyph_only=true/></td>
                        </tr>
                    });
                }
            }
            None
        })
        .collect();

    view! {
        <Title text="Basic Stack Operations and Formatting - Uiua Docs"/>
        <h1>"Basic Stack Operations and Formatting"</h1>
        <Hd id="the-stack">"The Stack"</Hd>
        <p>"In Uiua, all operations operate on a global stack. Lines of code are evaluated from "<A href="/docs/rtl">"right to left"</A>", top to bottom."</p>
        <p>"A number simply pushes its value onto the stack."</p>
        <Editor example="5"/>
        <Editor example="1 2 3"/>
        <p>"Operators pop values off the stack and push their results."</p>
        <p>"For example, "<Prim prim=Primitive::Add glyph_only=true/>" pops two values off the stack and pushes their sum."</p>
        <Editor example="+ 1 2"/>
        <p><Prim prim=Primitive::Mul glyph_only=true/>", of course, multiplies the two values instead."</p>
        <Editor example="+ 1 × 2 3" help={&["", "Try the arrows to see how the stack changes with each operation."]}/>
        <p>"In the editor, items that end up on the "<em>"top"</em>" of the stack are shown at the "<em>"bottom"</em>" of the output. This is so that consecutive lines of code show their outputs in the correct order."</p>
        <Editor example="5\n+1 2\n\"Hello, World!\"\n+1 @a"/>
        <p>"This orientation can be changed in the editor's settings. Click the ⚙️ icon in the top right corner of the editor to see them."</p>
        <p>"Operations can span multiple lines. Every line uses the same stack!"</p>
        <Editor example="1 2\n+\n5\n×"/>

        <Hd id="comments">"Comments"</Hd>
        <p>"Comments are denoted with "<code>"#"</code>" and run to the end of the line."</p>
        <Editor example="5 # This is a comment"/>
        <p>"Uiua does not have multiline comments."</p>

        <Hd id="formatting">"Formatting"</Hd>
        <p>"Most Uiua built-in functions use special Unicode characters. To type multiplication and division signs, you can use "<code>"*"</code>" and "<code>"%"</code>" respectively. Then, run the code to format the ASCII characters into Unicode."</p>
        <Editor example="# Click Run to format!\n%6 *3 8" help={&["", "⇡⇡⇡⇡ Click   "]}/>
        <p>"Most built-in functions have names you can type rather than symbols. Formatting works on these too. "<em><strong>"This is the primary way of entering Uiua's glyphs."</strong></em></p>
        <p>"Try formatting the lines below by clicking "<strong>"Run"</strong>"."</p>
        <Editor example="max sqrt 10 mod 10 pow 2 8"/>
        <Editor example="abs +`1 `2"/>
        <p>"You don't have to type the whole name, just enough to disambiguate it from others."</p>
        <Editor example="cei 1.5\nceil 1.5\nceili 1.5\nceilin 1.5\nceiling 1.5"/>
        <p>"You don't even have to remove spaces between built-in function names. The formatter will figure it out!"</p>
        <Editor example="roundsqrtpi"/>
        <p>"On this site, you can also click the ↧ symbol on any editor to show a palette of all the Uiua glyphs. You can then click on any glyph to insert it into the editor."</p>
        <p>"Here is a table of all the glyphs that are typed with ASCII characters that get converted to glyphs."</p>
        <table>
            <tr>
                <th>"Name"</th>
                <th>"ASCII"</th>
                <th>"Glyph"</th>
            </tr>
            {primitive_table}
            <tr>
                <td>"negative number"</td>
                <td><code>"`"</code></td>
                <td><code>"¯"</code></td>
            </tr>
        </table>
        <p>"As noted in the table, negative number literals are typed with the "<code>"`"</code>" character. This is because "<code>"-"</code>" is used for subtraction."</p>
        <Editor example="+ `1 `2"/>
        <p>"The formatter will align consecutive end-of-line comments. Try it out!"</p>
        <Editor example="%2 8 # Line\n@x # these\n1 # up"/>

        <h3>"Output Comments"</h3>
        <p>"A comment that starts with additional "<code>"#"</code>"s is an "<em>"output comment"</em>". The formatter replaces the text of an output comment with as many values from the stack as there are extra "<code>"#"</code>"s."</p>
        <p>"Click Run to try it out!"</p>
        <Editor example="1 2 3\n####\n+\n###\n+\n##"/>

        <Hd id="stack-functions">"Stack Functions"</Hd>
        <p>"There are a few functions that work on the stack itself. Some of these are critical and can be found scattered across all Uiua code."</p>
        <Hd id="dup"><Prim prim=Dup/></Hd>
        <p><Prim prim=Dup/>" duplicates the top item on the stack."</p>
        <p>"In general, functions do not leave their arguments on the stack. If you want to reuse a value, the most basic way is to "<Prim prim=Dup/>" it first."</p>
        <p>"For example, if you wanted to square a number, you could "<Prim prim=Dup/>" it, then "<Prim prim=Mul/>"."</p>
        <Editor example="×.4"/>
        <p><Prim prim=Dup/>" is often used in the examples on this site to show both the input and output of a function."</p>
        <Editor example="√.144"/>
        <br/>
        <Hd id="flip"><Prim prim=Flip/></Hd>
        <p><Prim prim=Flip/>" swaps the top two items on the stack."</p>
        <p>"This is useful when you want to call a function that takes two arguments, but the arguments are on the stack in the wrong order."</p>
        <p>"For example, if you wanted to get the reciprocal of a number, you would "<Prim prim=Div/>" "<code>"1"</code>" by it. But, if the number is already on the stack, you would need to use "<Prim prim=Flip/>"."</p>
        <Editor example="÷1 5"/>
        <Editor example="÷:1 5"/>
        <Editor example=":1 2 3 4 5"/>
        <br/>
        <Hd id="over"><Prim prim=Over/></Hd>
        <p><Prim prim=Over/>" is like "<Prim prim=Dup/>", but it duplicates the second item on the stack instead of the first."</p>
        <Editor example=",1 2 3 4"/>
        <Editor example="+×, 3 5"/>
        <br/>
        <Hd id="pop"><Prim prim=Pop/></Hd>
        <p><Prim prim=Pop/>" removes the top item from the stack."</p>
        <p>"This is useful when you want to discard a value that you do not need."</p>
        <Editor example="1 pop 2 3 4 ◌ 5 6"/>
        <Hd id="stack-and-trace"><Prim prim=Stack/>" and "<Prim prim=Trace/></Hd>
        <p><Prim prim=Stack/>" prints the entire stack."</p>
        <p>"It also attaches line and column numbers."</p>
        <p>"This is useful for debugging by inspecting the stack."</p>
        <Editor example="√+ ? .+ ? 1 ×3 4"/>
        <p><Prim prim=Trace/>" prints only the top item on the stack."</p>
        <Editor example="+1 ⸮ ×4 trace ×. -3 5"/>

        <Hd id="challenges">"Challenges"</Hd>
        <p>"At the end of most sections of this tutorial, there will be a few challenges to test your understanding."</p>
        <p>"The code you write will be run on multiple inputs and tested for correctness."</p>
        <p>"Each challenge has an example input and output followed by some test cases. There is also a hidden test case that your code is checked against, so make sure to think about edge cases!"</p>
        <p>"Remember that you can click the "<code>"↧"</code>" on the right side of the editor to see a list of all the glyphs."</p>
        <p>"Answers are available, but "<strong>"try to solve the challenges yourself first!"</strong></p>
        <p>"Some challenges have additional answers that use functions and concepts not yet covered in the tutorial, but which are more idiomatic."</p>
        <br/>

        <Challenge
            number=1
            prompt="adds 3 numbers"
            example="1 2 3"
            answer="++"
            tests={&["0 10 1", "10 ¯1 5", "0 5 1"]}
            hidden="6 7 8"/>

        <Challenge
            number=2
            prompt="divides the second number by the first but keeps the inputs on the stack"
            example="5 10"
            answer="÷,,"
            tests={&["6 24", "2 100", "17 51"]}
            hidden="8 32"/>

        <Challenge
            number=3
            prompt="subtracts the second number from the first then squares the result"
            example="10 1"
            answer="×.-:"
            tests={&["5 3", "9 2", "5 6"]}
            hidden="6 7"/>
    }
}

fn maybe_code<T: Display>(val: Option<T>) -> impl IntoView {
    if let Some(val) = val {
        view! {  <code>{ val.to_string() }</code> }.into_view()
    } else {
        view! {  "" }.into_view()
    }
}

fn primitive_rows(prims: impl IntoIterator<Item = Primitive>) -> Vec<impl IntoView> {
    prims
        .into_iter()
        .map(|p| {
            let ascii = p
                .ascii()
                .map(|s| s.to_string())
                .or_else(|| p.glyph().filter(|c| c.is_ascii()).map(|c| c.to_string()));
            view! {
                <tr>
                    <td><Prim prim=p/></td>
                    <td>{maybe_code( ascii)}</td>
                    <td>{view!( <code>{p.args()}</code>)}</td>
                </tr>
            }
        })
        .collect()
}

#[component]
fn TutorialMath() -> impl IntoView {
    use Primitive::*;
    let math_table = primitive_rows([
        Add, Sub, Mul, Div, Mod, Pow, Log, Neg, Abs, Ceil, Floor, Round, Sqrt, Sign,
    ]);
    let comp_table = primitive_rows([
        Eq, Ne, Lt, Gt, Le, Ge, Min, Max, Floor, Ceil, Round, Sin, Atan,
    ]);

    view! {
        <Title text="Math and Comparison - Uiua Docs"/>
        <h1>"Math and Comparison"</h1>
        <p>"Uiua supports all the basic math operations as well as comparison, min/max, and rounding."</p>
        <div id="ascii-glyphs">
            <table class="bordered-table">
                <tr>
                    <th>"Function"</th>
                    <th>"ASCII"</th>
                    <th>"Args"</th>
                </tr>
                {math_table}
            </table>
            <table class="bordered-table">
                <tr>
                    <th>"Function"</th>
                    <th>"ASCII"</th>
                    <th>"Args"</th>
                </tr>
                {comp_table}
            </table>
        </div>
        <p>"Most of these are used mostly how you might think."</p>
        <Editor example="+2 5"/>
        <Editor example="↥2 5"/>
        <Editor example="ⁿ2 5"/>
        <Editor example="⌈2.5"/>
        <Editor example="√4"/>
        <p>"Uiua has no boolean type. Comparison operators return "<code>0</code>" for false and "<code>1</code>" for true."</p>
        <Editor example="=2 5"/>
        <Editor example="=2 2"/>
        <p>"One thing to note is that non-commutative operators work backwards."</p>
        <p>"This is so you can think of the operator and the second number as a single unit."</p>
        <Editor example="-2 5" help={&["", "What is 5 \"minus 2\"?"]}/>
        <Editor example="<2 5" help={&["", "Is 5 \"less than 2\"?"]}/>
        <Editor example="÷2 5" help={&["", "What is 5 \"divided by 2\"?"]}/>
        <p>"Because of how stack operations work, you can delay operations until after all the arguments are on the stack."</p>
        <Editor example="×++1 2 3 4" help={&["", "Click the arrows to see how the expression is built up"]}/>
        <p>"This is not special syntax. All the numbers are pushed to the stack, then the operators work on them."</p>
        <p>"Remember that you can type the names of operators and then run to format them."</p>
        <Editor example="# Click Run to format!\nmax sqrt2 mod10 abs`31" help={&["", "⇡⇡⇡⇡ Click   "]}/>

        <Hd id="adicity">"Adicity"</Hd>
        <p>"Some programming languages use the terms \"unary\" and \"binary\" to refer to functions that take one or two arguments respectively. While these are the Latin terms, many array languages, including Uiua, prefer to use the Greek terms \"monadic\" and \"dyadic\"."</p>
        <p>"As you read Uiua's documentation, you will see these terms used to describe functions (and modifiers)."</p>
        <p>"For example, "<Prim prim=Sqrt/>" is a monadic function, and "<Prim prim=Add/>" is a dyadic function."</p>
        <p>"On this site, monadic functions are in "<span class="monadic-function">"green"</span>" and dyadic functions are in "<span class="dyadic-function">"blue"</span>"."</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="for arguments A, B, and C, computes (A + B) × C"
            example="1 2 3"
            answer="×+"
            tests={&["2 2 2", "5 7 2", "3 ¯1 ¯1"]}
            hidden="6 7 8"/>

        <Challenge
            number=2
            prompt="calculates the hypotenuse of a right triangle with sides A and B (√(A² + B²))"
            example="3 4"
            answer="√+×.:×."
            best_answer="⌵ℂ"
            tests={&["12 9", "5 12", "6 8"]}
            hidden="5 3"/>
    }
}

#[component]
fn TutorialArrays() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Arrays - Uiua Docs"/>
        <h1>"Arrays"</h1>
        <p>"Uiua is, first and foremost, an array language. The only composite data type is the multidimensional array. Arrays have a lot of nice properties, and the language's built-in functions are designed to make it easy to work with them. If you've only ever programmed in non-array languages, then this will be a completely foreign paradigm. In most array languages, most data structures and control flow are replaced with operations on arrays."</p>

        <Hd id="creating-arrays">"Creating Arrays"</Hd>
        <p>"Other than with functions, Uiua has two ways to create arrays. They are called "<em>"strand notation"</em>" and "<em>"stack notation"</em>"."</p>
        <p><strong>"Strand notation"</strong>" uses underscores to connect elements."</p>
        <Editor example="1_2_3"/>
        <Editor example="\"Hello\"_\"World\""/>
        <p>"Strand notation is good when you want to create short and/or simple arrays. For longer or more complex arrays, you can use stack notation."</p>
        <p><strong>"Stack notation"</strong>" uses "<code>"[]"</code>" brackets to group elements."</p>
        <Editor example="[1 2 3]"/>
        <Editor example="[¯5 37 42 π]"/>
        <p>"What's cool about stack notation is that it is "<em>"not"</em>" just a way to list elements. The code between the brackets runs from right to left as it normally would. When it is done, any items on the stack higher than when it started are put into the array. This gives you some cool ways to create arrays."</p>
        <p>"Remember that "<Prim prim=Dup/>" duplicates the top item on the stack."</p>
        <Editor example="[...5]"/>
        <Editor example="[×2.×2.×2.×2 .2]"/>
        <Editor example="[+1 2 +3 4]"/>
        <p>"Any functions inside the brackets will \"pull in\" their arguments from outside if there are not enough inside."</p>
        <Editor example="[+] 1 9"/>
        <Editor example="[...] 7"/>
        <Editor example="[+×2] 20 2"/>
        <p>"You can also use stack notation to make multidimensional arrays."</p>
        <Editor example="[1_2_3 4_5_6]"/>
        <Editor example="[...[1 2 3]]"/>
        <p>"Unlike strand notation, stack notation may span multiple lines. The lines are still executed right-to-left, but they are executed bottom-to-top so that the arrays come out the same way they look in the code."</p>
        <Editor example="\
[1 2 3
 4 5 6
 7 8 9]"/>
        <Editor example="\
[[1 2 3]
 [4 5 6]
 [7 8 9]]"/>
        <p>"More precisely, stack notation "<Prim prim=Couple/>"s the first two stack items created between the "<code>"[]"</code>"s and "<Prim prim=Join/>"s the rest to that coupling. You may see this refered to in error messages."</p>

        <Hd id="shape-len"><Prim prim=Shape/>" and "<Prim prim=Len/></Hd>
        <p>"Other than their data, arrays also have a property called their "<strong>"shape"</strong>". Shape is a list of non-negative integers that describes the array's size along each of its axes."</p>
        <p>"We can get the array's shape with the "<Prim prim=Shape/>" function. It's a triangle because a triangle is a shape."</p>
        <Editor example="△[1 2 3]"/>
        <Editor example="△5"/>
        <Editor example="△[[1 2 3] [4 5 6]]"/>
        <Editor example="△[...[1 2 3]]"/>
        <p>"Arrays with 0 dimensions (an empty "<Prim prim=Shape/>") are called "<strong>"scalars"</strong>"."</p>
        <p>"Arrays with 1 dimension are often called "<strong>"lists"</strong>" or "<strong>"vectors"</strong>"."</p>
        <p>"Arrays with 2 dimensions are often called "<strong>"tables"</strong>" or "<strong>"matrices"</strong>"."</p>
        <p>"While there are not common names for arrays with 3 or more dimensions, Uiua supports arrays with an arbitrary number of axes."</p>
        <p>"The first element of the shape is the number of "<em>"rows"</em>" of the array. "<em>"Rows"</em>" does not refer just to the rows of a matrix or table. It is the groups of elements along the leading axis of the array. For lists, this is just the individual elements. For matrices, it is the rows as you might traditionally think of them. But arrays with a higher number of dimensions have rows as well. For example, in an array with 3 dimensions, each row is a matrix."</p>
        <p>"From shape we can derive two closely-related properties called "<strong>"length"</strong>" and "<strong>"rank"</strong>"."</p>
        <p><Prim prim=Len/>" is the number of rows in the array. Length is always equal to the first number in the shape (or 1 if the shape is empty)."</p>
        <p><strong>"Rank"</strong>" is the number of dimensions of the array. It is equivalent to the "<Prim prim=Len/>" of the "<Prim prim=Shape/>"."</p>
        <Editor example=" △[1_2_3 4_5_6 7_8_9]\n ⧻[1_2_3 4_5_6 7_8_9]\n⧻△[1_2_3 4_5_6 7_8_9] # Rank"/>

        <Hd id="output">"Pretty Array Output"</Hd>
        <p>"The online editor and native interpreter both pretty-print any values that remain on the stack when a program is finished. (This can be invoked manually using the "<Prim prim=Sys(SysOp::Show)/>" function.)"</p>
        <p>"To understand how the pretty-printed output corresponds to the actual array, we can use "<Prim prim=Reshape/>" to create a multidimensional array. "<Prim prim=Reshape/>" uses its first argument as a new shape for its second argument."</p>
        <p>"Here, we create a "<Prim prim=Range/>" array of all the numbers up to "<code>"24"</code>" and turn it into a 3-dimensional array with the shape "<code>"[2 3 4]"</code>"."</p>
        <Editor example="↯2_3_4 ⇡24"/>
        <p>"Notice there are "<code>"2"</code>" big cells, each with "<code>"3"</code>" rows of "<code>"4"</code>" elements."</p>
        <p>"This expands to any number of dimensions. The more dimensions, the more space between the cells representing earlier axes."</p>
        <Editor example="↯2_3_2_5 ⇡60"/>

        <Hd id="pervasion">"Pervasion"</Hd>
        <p>"Most operations that apply to scalars are what is called "<em>"pervasive"</em>" when it comes to arrays. This means that the operation automatically applies to every item in the array."</p>
        <Editor example="+1 1_2_3"/>
        <Editor example="√[4 9 16]"/>
        <Editor example="+1_2_3 4_5_6"/>
        <p>"When doing a pervasive operation on two arrays, the shape of one array must be the "<em>"prefix"</em>" of the shape of the other."</p>
        <Editor example="+[1 2] [3 4 5]"/> // Should fail
        <p>"Notice here that the shape of the first array is a prefix of the shape of the second array."</p>
        <Editor example="△10_20\n      △[3_4_5 6_7_8]\n+10_20 [3_4_5 6_7_8]"/>
        <p>"If you want to do some pervasive operation on arrays whose shapes do not match, you can set a default value with "<Prim prim=Fill/>". Any places where the shapes don't match will be filled in with that value."</p>
        <Editor example="⬚10+ [1 2] [3 4 5 6 7]"/>
        <p><Prim prim=Fill/>" can be used in a lot of other cases. See its documentation for more."</p>
        <p>"Pervasive operations are optimized in the interpreter to be very fast. You should prefer to use them whenever possible."</p>

        <Hd id="useful-array-operations">"Useful Array Operations"</Hd>
        <p>"You don't need to memorize all of these right now. This is just a brief introduction to some of the array operations so that you won't be surprised when you see them later."</p>
        <p>"If you ever see a glyph that you don't recognize in an example, you can hold ctrl/⌘ and mouse over it in the editor to learn its name."</p>
        <p>"You can ctrl/⌘-click any glyph in the editor to see its documentation."</p>
        <p>"You can also click the names of functions in the site text to see their documentation."</p>
        <p><Prim prim=Couple/>" turns two arrays into rows of a new array."</p>
        <Editor example="⊟ 1_2_3 [4 5 6]"/>
        <p><Prim prim=First/>" gets the first row of an array."</p>
        <Editor example="⊢ [4 7 1]"/>
        <Editor example="⊢ [1_2 3_4 5_6]"/>
        <p><Prim prim=Reverse/>" reverses the rows of an array."</p>
        <Editor example="⇌ [4 7 1]"/>
        <Editor example="⇌ [1_2 3_4 5_6]"/>
        <p><Prim prim=Rotate/>" rotates the rows of an array by some amount."</p>
        <Editor example="↻2 [1 2 3 4 5]"/>
        <p><Prim prim=Deshape/>" flattens an array into a 1D array."</p>
        <Editor example="♭ .[1_2 3_4 5_6]"/>
        <p><Prim prim=Take/>" and "<Prim prim=Drop/>" isolate part of an array."</p>
        <Editor example="↙3 [1 2 3 4 5]\n↘3 [1 2 3 4 5]"/>
        <p><Prim prim=Pick/>" indexes an array. Longer indices index deeper into the array."</p>
        <p>"Uiua is 0-indexed."</p>
        <Editor example="⊡2 [3 8 4 1]"/>
        <Editor example="⊡1   [1_2_3 4_5_6]\n⊡1_1 [1_2_3 4_5_6]"/>
        <p><Prim prim=Select/>" uses a list of indices to select rows of an array."</p>
        <Editor example="⊏ [0 2 1 1 2] ↯3_3⇡9"/>
        <Editor example="⊏[3 5 0 1 7 8 9 5 1 2 5 3 10] \"their sinks\""/>

        <Hd id="array-model">"The Array Model"</Hd>
        <p>"For curious array aficionados, Uiua uses an array model resembling "<a href="https://aplwiki.com/wiki/Box">"J's Boxed array model"</a>"."</p>
        <p>"All arrays are flat and homogenous. Arrays always have a rectangular shape, meaning that all rows along an axis always have the same length. Different types of data, like numbers and characters, cannot be mixed in the same array."</p>
        <p>"However, there is an escape hatch for when you really want jagged, nested, or mixed-type arrays. In Uiua, an array of heterogeneous values can be simulated with an array of "<em>"boxes"</em>"."</p>
        <p>"The array below cannot be constructed normally because its rows have different "<Prim prim=Shape/>"s."</p>
        <Editor example="[1 2 [7 8 9]]"/> // Should fail
        <p>"By using "<Prim prim=Box/>", we can turn any value into a "<strong>"box"</strong>" that contains that value. We can then put these boxes into an array together."</p>
        <Editor example="[□1 □2 □[7 8 9]]"/>
        <p>"The "<code>"{}"</code>"s in the output hint at some syntax that will be introduced shortly."</p>
        <p><Prim prim=Un/><Prim prim=Box/>" extracts a "<Prim prim=Box/>"ed value."</p>
        <Editor example="°□ .□[1 2 3]"/>
        <p>"The "<code>"⟦⟧"</code>"s indicate that a list is "<Prim prim=Box/>"ed."</p>
        <p><Prim prim=Box/>"ed strings also have special output delimiters, using "<code>"⌜⌟"</code>"s."</p>
        <Editor example="□\"banana\""/>
        <p>"Having to write "<Prim prim=Box glyph_only=true/>" everywhere is annoying, and so..."</p>

        <Hd id="nested-arrays">"Nested Arrays"</Hd>
        <p>"Uiua has a special syntax for making arrays where every item is "<Prim prim=Box/>"ed."</p>
        <p>"Using "<code>"{}"</code>"s instead of "<code>"[]"</code>"s for stack array notation will automatically "<Prim prim=Box/>" every item."</p>
        <Editor example="{1 2 [7 8 9]}"/>
        <p>"This is very useful for making lists of strings."</p>
        <Editor example=r#"["Uiua" "APL" "J" "BQN" "K" "Q"] # Fails"#/>
        <Editor example=r#"{"Uiua" "APL" "J" "BQN" "K" "Q"} # Works!"#/>
        <p>"Functions that require their arguments to have matching types may require "<Prim prim=Box/>"ing an argument."</p>
        <p>"For example, to check if a string is in a list of "<Prim prim=Box/>"ed strings with "<Prim prim=MemberOf/>", you would need to "<Prim prim=Box/>" the string first."</p>
        <Editor example=
r#"Langs ← {"Uiua" "APL" "J" "BQN" "K" "Q"}
∈ Langs □"APL""#/>
        <p>"Pervasive functions work through boxes and preserve the maximum "<Prim prim=Box/>" depth of their arguments."</p>
        <Editor example="¯ 1\n¯ □1\n¯ □□1"/>
        <Editor example="+1 4\n+1 □4\n+1 □□4\n+□□1 □4"/>
        <Editor example="×10 {1_2_3 4_5 6}"/>
        <p>"There is an exception for comparison functions, which compare lexicographically."</p>
        <Editor example=r#"=  [1 2 3]  [1 2 5]
= □[1 2 3] □[1 2 5]
>  [1 2 3]  [1 2 5]
> □[1 2 3] □[1 2 5]
>  "banana"  "orange"
> □"banana" □"orange"
> □"banana"  "orange""#/>
        <p>"Non-pervasive functions often require "<Prim prim=Un/><Prim prim=Box/>"ing the arguments to get at the value you want."</p>
        <p>"Consider this difference:"</p>
        <Editor example="△    ⊢{1_2_3 5_6}\n△ °□ ⊢{1_2_3 5_6}"/>

        <p>"For more about working with box arrays, see "<Prim prim=Box/>"'s documentation."</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="adds an array to its reverse"
            example="1_2_5"
            answer="+⇌."
            tests={&["3_1_7", "↯2_4⇡8", "5"]}
            hidden="0_1_2_6"/>

        <Challenge
            number=2
            prompt="creates a matrix of 0's with as many rows as the first argument and as many columns as the second argument"
            example="3 4"
            answer="↯:0⊟"
            tests={&["2 7", "3 3", "1 8"]}
            hidden="1 1"/>

        <Challenge
            number=3
            prompt="adds a 1-row leading axis to an array"
            example="[1 2 3]"
            answer="↯⊂1△."
            best_answer="¤"
            tests={&["1_3_1_5", "5", "↯2_3⇡6"]}
            hidden="1_1_1_1_1"/>

        <Challenge
            number=4
            prompt="appends the first row of the first argument to the second argument"
            example="[1 2 3] 4_5_6"
            answer="⊂⊢"
            tests={&["3_3 2_2", "[1_2_3 4_5_6] +10↯3_3⇡9", "[2 4 3] [9 9 9 9 9 1]"]}
            hidden="↯2_3_4⇡24 ↯3_4⇡12"/>

        <Challenge
            number=5
            prompt="splits an array into its first row and the rest of its rows"
            example="1_2_3_4"
            answer="⊢:↘1."
            best_answer="°⊂"
            tests={&["[27 9 3 1]", "↯4_3⇡12"]}
            hidden="[5]"/>

        <Challenge
            number=6
            prompt="boxes two strings and puts them in an array"
            example="\"Hello\" \"World\""
            answer="⊟□:□:"
            best_answer="{⊙∘}"
            tests={&["\"ui\" \"ua\"", "\"dog\" \"cat\""]}
            hidden="\"a\" \"b\""/>
    }
}

#[component]
fn TutorialTypes() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Types - Uiua Docs"/>
        <h1>"Types"</h1>
        <p>"Every value in Uiua is an array. However, different arrays on the stack can have different "<em>"types"</em>" of items. Every element of an array is always the same type. Unlike some other array programming languages, Uiua arrays cannot have elements of different types."</p>
        <p>"There are four types of arrays:"</p>
        <ul>
            <li><strong>"Number"</strong></li>
            <li><strong>"Complex"</strong></li>
            <li><strong>"Character"</strong></li>
            <li><strong>"Box"</strong></li>
        </ul>

        <Hd id="numbers">"Numbers"</Hd>
        <p>"Numbers are decimal numbers with floating precision. They use the IEEE-754 double-precision floating-point format."</p>
        <Editor example="[5 6e3 0 3.2 3/4 ¯1.1 π ∞]"/>
        <p>"Most math operations can only be applied to numbers."</p>
        <p>"In cases where a number with a fractional part has repeating decimals, or when floating-point errors create tiny differences, the number will be shown with repeated decimal digits replaced by a "<code>"…"</code>"."</p>
        <Editor example="1/3\n1/12\n1/24\n+ 0.1 0.2"/>
        <p>"Even though numbers can have a fractional part, many built-in functions require whole numbers. These functions will return an error if given a non-whole number."</p>
        <p>"One such example is "<Prim prim=Pick/>"."</p>
        <Editor example="⊡ 2 [4 7 9 1 0]"/>
        <Editor example="⊡ 3.1 [4 7 9 1 0]"/> // Should fail
        <p>"If you want to convert a number to a whole number, you can use "<Prim prim=Floor/>", "<Prim prim=Ceil/>", or "<Prim prim=Round/>"."</p>

        <Hd id="complex-numbers">"Complex Numbers"</Hd>
        <p>"Complex numbers can be created with the "<Prim prim=Complex/>" function."</p>
        <Editor example="ℂ 3 5"/>
        <Editor example="ℂ [1 2 3] [4 5 6]"/>
        <p>"While complex numbers support all the same math operations as normal numbers, they are a distinct type and cannot be used in place of normal numbers."</p>
        <p>"You can convert a complex number to a normal number with "<Prim prim=Abs/>"."</p>
        <Editor example="⌵ ℂ3 4"/>
        <p>"You can normalize a complex number to a unit vector with "<Prim prim=Sign/>"."</p>
        <Editor example="± ℂ3 4"/>
        <p><Prim prim=Sqrt/>" only returns a complex number if it is called on a complex number. Beware of floating-point errors."</p>
        <Editor example="√  ¯4\n√ℂ0¯4"/>
        <p>"See "<Prim prim=Complex/>"'s docs for more details."</p>
        <p>"Comparing complex numbers for equality returns a normal number."</p>
        <Editor example="= i ℂ0 1\n= i ℂ1 1"/>
        <p>"Comparing complex numbers for order returns a component-wise comparison."</p>
        <Editor example="< i ℂ¯1 1\n≥ i ℂ1 1"/>

        <Hd id="characters">"Characters"</Hd>
        <p>"Characters are represented as 32-bit Unicode codepoints."</p>
        <p>"Character literals, denoted with a preceding "<code>"@"</code>", create rank 0 (scalar) character arrays."</p>
        <Editor example="@a @b"/>
        <Editor example="[@u @i @u @a]"/> // Should fail
        <p>"Characters like newline or null need to be escaped with "<code>"\\"</code>", but spaces do not."</p>
        <Editor example="@\\r @\\0 @ "/>
        <p>"If you don't like the significant whitespace of "<code>"@ "</code>", "<code>"@\\s"</code>" is also space."</p>
        <p>"As noted in the advice diagnostic above, string literals, delimited by "<code>"\""</code>"s, create rank-1 character arrays."</p>
        <Editor example="△.\"Hello, World!\""/>
        <p>"You can make raw strings, which do not require escaping, with a "<code>"$"</code>" followed by a space."</p>
        <p><Prim prim=Sys(SysOp::Print)/>" pretty-prints a value."</p>
        <Editor example="&p $ \"How are you?\" she asked."/>
        <p>"Raw strings that follow each other form multi-line strings."</p>
        <Editor example="$ Hello\n$ World!"/>
        <p>"This style of string is useful when your string contains a lot of quotes that you don't want to escape."</p>
        <Editor example="$ And then she was like, \"No way!\"\n$ And I was like, \"Way...\""/>
        <p>"Characters in character or string literals can also be specified with 2 or 4 hex digits by using escape codes "<code>"\\x"</code>" and "<code>"\\u"</code>" respectively."</p>
        <Editor example="\"\\x41\\x42\\x43\""/>
        <Editor example="@\\u2665"/>
        <p>"Longer (or shorter) sequences can be specified between "<code>"{}"</code>"s after a "<code>"\\u"</code>"."</p>
        <Editor example="@\\u{1f600}"/>
        <p>"Note that these escape sequences do not work in raw strings."</p>
        <br/>

        <Hd id="character-arithmetic">"Character Arithmetic"</Hd>
        <p>"Characters and numbers exist in an "<a href="https://en.wikipedia.org/wiki/Affine_space">"affine space"</a>", the same as in "<a href="https://mlochbaum.github.io/BQN/doc/arithmetic.html#character-arithmetic">"BQN"</a>"."</p>
        {
            let number = || view!(<span class="number-literal">"number"</span>);
            let character = || view!(<span class="string-literal-span">"character"</span>);
            view! {
                <p>"You can "<Prim prim=Add/>" "{number}"s and "{character}"s to get another "{character}"."</p>
                <p>"You can "<Prim prim=Sub/>" a "{number}" from a "{character}" to get another "{character}"."</p>
                <p>"You can "<Prim prim=Sub/>" two "{character}"s to get a "{number}"."</p>
                <p>"You can "<Prim prim=Mul/>" or "<Prim prim=Div/>" a "{character}" by a "{number}" to possibly toggle its case."</p>
                <p><em>"No"</em>" other binary arithmetic operations can be done on "{character}"s."</p>
            }
        }
        <Editor example="+1 @a"/>
        <Editor example="-8 \"Uiua\""/>
        <Editor example="-@a @z"/>
        <Editor example="× [1 ¯5 0 ¯2] \"uiua\""/>
        <Editor example="+@a @b"/> // Should fail
        <p><Prim prim=Sign/>" gives the case of a character. It gives "<code>"1"</code>" for uppercase, "<code>"¯1"</code>" for lowercase, and "<code>"0"</code>" for caseless characters."</p>
        <Editor example="± \"Hello, World!\""/>
        <p><Prim prim=Abs/>" uppercases a character."</p>
        <Editor example="⌵ \"Hello, World!\""/>
        <p><Prim prim=Neg/>" toggles the case of a character."</p>
        <Editor example="¯ \"Hello, World!\""/>
        <p>"Use "<Prim prim=Neg/>" and "<Prim prim=Abs/>" together to lowercase a character."</p>
        <Editor example="¯⌵ \"Hello, World!\""/>

        <Hd id="boxes">"Boxes"</Hd>
        <p>"Boxes are containers that can wrap an array of any type or shape. Multiple boxes can be put in the same array, no matter their contents."</p>
        <p>"Boxes can be created either by using the "<Prim prim=Box/>" function or with boxing array notation between "<code>"{}"</code>"s."</p>
        <Editor example="□5"/>
        <Editor example="□[1 2 3]"/>
        <Editor example="□\"Hello!\""/>
        <Editor example="{\"cat\" 5}"/>

        <Hd id="type-agreement">"Type agreement"</Hd>
        <p id="type-agreement">"For functions that work on the structure of arrays rather than their values, the types of the arrays must match."</p>
        <Editor example="⊂ 1_2 3"/>
        <Editor example="⊟ \"Hello\" \"World\""/>
        <Editor example="⊟ 1_2_3 \"dog\""/> // Should fail
        <p>"There is an exception for boxes. Any box can be put in an array with a non-box. In this case, the non-box will be "<Prim prim=Box/>"ed first."</p>
        <Editor example="⊟ 5 □[1 2 3]"/>

        <Hd id="empty-arrays">"Empty Arrays"</Hd>
        <p>"The type of an array that is constructed with no elements depends on the syntax used to construct it. Its shape is always "<code>"[0]"</code>"."</p>
        <p>"We can use the "<Prim prim=Type/>" function to get the type of an array. "<code>"0"</code>" corresponds to real numbers, "<code>"1"</code>" to complex numbers, "<code>"2"</code>" to characters, and "<code>"3"</code>" to boxes."</p>
        <Editor example="type []"/>
        <Editor example="type \"\""/>
        <Editor example="type {}"/>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="increments the first character of a string"
            example="\"`rray\""
            answer="⊂:↘1:+1⊢."
            best_answer="⍜⊢+:1"
            tests={&["\"Xou're\"", "\"coing\"", "\"freat!\""]}
            hidden="\"abc\""/>
    }
}

#[component]
fn TutorialBindings() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Bindings - Uiua Docs"/>
        <h1>"Bindings"</h1>
        <p>"Bindings are global names that can be given to Uiua values. They are denoted with "<code>"←"</code>", which the formatter will convert from "<code>"="</code>" when appropriate."</p>
        <Editor example="a = 3\nb ← 5\n+ a b" help={&["", "Try running to format the ="]}/>
        <p>"Valid binding names can be made up of any sequence of uppercase or lowercase alphabetic characters OR a single non-alphanumeric character that is not already used for a Uiua function or syntax."</p>
        <Editor example="NumOne ← 1\nNumTwo ← 2\n😀 ← \"happy\""/>
        <p><em>"Warning"</em>": It is not guaranteed that any particular non-alphanumeric character will not be used for a built-in function in the future. Use them at your own risk. Emojis are safe though."</p>
        <p>"Unlike most programming languages, binding names in Uiua "<em>"cannot"</em>" contain numbers or underscores."</p>
        <Editor example="Variable_1 ← 5"/> // Should fail
        <p>"Bindings "<em>"can"</em>" contain subscript numbers. These will format from "<code>"__"</code>" followed by some digits. Try formatting the example below!"</p>
        <Editor example="X__1 = 5\nSha__256 = \"TODO\""/>
        <p>"Bindings are case-sensitive."</p>
        <p>"The parser can sometimes mistake all-lowercase binding names for unformatted built-in functions."</p>
        <p>"Here, the parser thinks that "<code>"part"</code>" is "<Prim prim=Partition/>"."</p>
        <Editor example="part = 5" help={&["", "Run to format and reveal why this does not work"]}/>
        <p>"Binding names with 2 or more characters should be "<A href="https://en.wikipedia.org/wiki/Camel_case">"PascalCase (also known as upper CamelCase)"</A>" to avoid this issue."</p>
        <Editor example="Part = 5\n*2 Part"/>
        <p>"Bindings run the code to the right of the "<code>"←"</code>", then pop the top value off the stack and bind it to the name on the left."</p>
        <p>"Note, though, that an empty right side is perfectly valid! This means you can bind values that were created on previous lines."</p>
        <Editor example="×6 7\nAnswer ←\n[Answer]"/>

        <Hd id="binding-functions">"Binding Functions"</Hd>
        <p>"If the code on the right side of the "<code>"←"</code>" requires more than 0 values to be on the stack, then instead of evaluating its right side immediately, the right side will be bound as a function."</p>
        <p>"This is how you make named functions in Uiua."</p>
        <Editor example="F ← +1\nF 5"/>
        <Editor example="Cube ← ××..\nCube 6"/>
        <Editor example="👋 ← ⊂\"Hello, \"\n👋 \"World!\""/>
        <p>"If the code on the right side takes 0 arguments but you still want it to be a function, it must be surrounded by "<code>"()"</code>"s."</p>
        <p>"Notice how the first example here gives the same value every time, while the second one does not."</p>
        <Editor example="F ← ⚂\nF F F"/>
        <Editor example="F ← (⚂)\nF F F"/>
        <p>"The "<A href="/tutorial/functions">"next section"</A>" discusses functions in more detail."</p>
    }
}

#[component]
fn TutorialFunctions() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Modifiers and Functions - Uiua Docs"/>
        <h1>"Modifiers and Functions"</h1>

        <Hd id="modifiers">"Modifiers"</Hd>
        <p>"Modifiers are functions that take other functions as arguments. If you immediately follow a modifier with its function arguments, the functions will be called inside the modifier rather than outside."</p>
        <p>"For example, "<Prim prim=Reduce/>" applies a function \"between\" all rows of an array."</p>
        <p><Prims prims={[Reduce, Add]}/>" is therefore the sum of all the rows of an array."</p>
        <Editor example="/+ 1_2_3_4"/>
        <p><Prim prim=Scan/>" is similar, but it returns all the intermediate results."</p>
        <Editor example="\\+ 1_2_3_4"/>
        <p><Prim prim=Rows/>" applies a function to each row of an array."</p>
        <p>"For example, "<Prims prims=[Reduce, Add]/>" adds each row of a matrix to the next, effectively summing along the columns."</p>
        <p><Prims prims=[Rows, Reduce, Add]/>" sums each row itself."</p>
        <Editor example="    [1_2_3 4_5_6 7_8_9]\n /+ [1_2_3 4_5_6 7_8_9]\n≡/+ [1_2_3 4_5_6 7_8_9]"/>
        <p><Prim prim=Table/>" applies a function between all combinations of rows of two arrays. This is sometimes called the "<em>"outer product"</em>"."</p>
        <Editor example="⊞+ [5 6 7 8] [10 20 30 40]"/>
        <p>"In the same way that \"monadic\" and \"dyadic\" functions refer to functions that take one or two array arguments respectively, \"monadic\" and \"dyadic\" "<em>"modifiers"</em>" refer to modifiers that take one or two "<em>"functions"</em>" respectively."</p>
        <p>"On this site, monadic modifiers are in "<span class="monadic-modifier">"yellow"</span>" and dyadic modifiers are in "<span class="dyadic-modifier">"purple"</span>"."</p>
        <p>"The main docs page has "<A href="/docs/modifier">"a list"</A>" of all of the built-in modifiers."</p>

        <Hd id="inline-functions">"Inline Functions"</Hd>
        <p>"In addition to creating a new function with a capitalized binding name, as discussed in the "<A href="/tutorial/bindings">"previous section"</A>", functions in Uiua can also be created by surrounding code with "<code>"()"</code>"s."</p>
        <p>"This is usually only necessary when you need to call multiple functions within a modifier."</p>
        <p>"For example, if you wanted to get the last element of each row of an array, you could use "<Prim prim=Rows/>"."</p>
        <Editor example="≡(⊢⇌) .[2_5_3 0_2_1 0_0_2]"/>
        <p>"Inline functions may span multiple lines. Unlike multiline stack notation arrays, which run bottom-to-top, multiline inline functions run top-to-bottom as other code does."</p>
        <Editor example="\
X ← (
  ⊞=.⇡ # First this line runs
  ↥⇌.  # Then this one
)
X 5"/>
        <p>"Output comments inside inline functions will show the values on the stack for each time the function is called. Try it out!"</p>
        <Editor example="\
F ← (
  ### Run to see values here!
  +×.
  ##
)
F 3 5
F 2 9
F 10 11"/>

        <Hd id="local-bindings">"A Note on Local Bindings"</Hd>
        <p>"Bindings in Uiua can "<em>"only"</em>" be global. There is no way to give a name to a value within an inline function. A "<code>"←"</code>" inside "<code>"()"</code>"s is a syntax error."</p>
        <p>"This is a deliberate design decision. It forces you to write tacit code, a.k.a. code with functions that do not mention their arguments. Uiua is designed to make writing tacit code as workable as possible. "<em>"How"</em>" it does this will be discussed in "<A href="/tutorial/advancedstack">"later"</A>" "<A href="/tutorial/advancedarray">"sections"</A>"."</p>

        <Hd id="format-strings">"Format Strings"</Hd>
        <p>"Prefixing a string with a "<code>"$"</code>" creates a format string. A format string is a special kind of function. It takes an argument for each "<code>"_"</code>" in the string and replaces it with the stringified version."</p>
        <Editor example="\"World\"\n$\"Hello, _!\""/>
        <Editor example="Greet ← $\"Hello, _!\"\nGreet \"user\""/>
        <Editor example="x ← 5\n$\"x = _\" x"/>
        <Editor example="$\"_, _, and _\" 1 2 3"/>
        <p>"If you need to use a literal "<code>"_"</code>", you can escape them with "<code>"\\"</code>"."</p>
        <Editor example="$\"\\__\\_\" 27"/>
        <p>"Raw strings can be made format strings by adding an additional "<code>"$"</code>"."</p>
        <Editor example="+,, 1 2\n&p $$ What are two numbers that add up to _?\n   $$ _ and _ do!"/>
        <p><code>"_"</code>"s still need to be escaped in raw format strings."</p>
        <Editor example="1 2 3\n$$ _\\__\\__"/>
        <p>"Because format strings are just functions, you can use them with modifiers like "<Prim prim=Reduce/>". This is a common way to join a list of "<Prim prim=Box/>"ed strings."</p>
        <Editor example="/$\"_ _\" {\"Separated\" \"by\" \"spaces\"}"/>

        <Hd id="stack-signatures">"Stack Signatures"</Hd>
        <p>"Bindings and inline functions can have a "<em>"stack signature"</em>" declared with a "<code>"|"</code>" followed by 1 or 2 numbers separated by a "<code>"."</code>". The first number is the number of arguments the function pops from the stack. The second number is the number of values the function pushes to the stack."</p>
        <p>"The second number is optional. If it is not given, it is assumed to be 1."</p>
        <p>"In bindings, the "<code>"|"</code>" comes after the "<code>"←"</code>". In inline functions, it comes after the "<code>"("</code>"."</p>
        <Editor example="TimesThree ← |1.1 ×3\nTimesThree 7"/>
        <Editor example="TimesThree ← |1   ×3\nTimesThree 7"/>
        <Editor example="∵(|2.1 ⊟.×) 1_2_3 4_5_6"/>
        <p>"Stack signatures are useful for documenting functions to make sure that they are used correctly."</p>
        <p>"A signature declaration is "<em>"required"</em>" if the function's signature cannot be inferred. The compiler can usually infer a function's signature unless you are doing something weird that it cannot reason about."</p>
        <p>"In addition, an error is thrown if a function's signature can be inferred and the inferred signature does not match the declared signature. This can help validate that a function is correct."</p>
        <Editor example="≡(|2 ↻.) 1_2_3 ↯3_3⇡9"/> // Should fail
        <p>"If the compiler cannot derive the stack signature of a function and you give it one which is "<em>"wrong"</em>", the function will throw an error at runtime."</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="calculates the product of the first n positive integers"
            example="5"
            answer="/×+1⇡"
            tests={&["10", "0", "6"]}
            hidden="3"/>

        <Challenge
            number=2
            prompt="adds each column of a matrix to the next"
            example="[1_2_3 4_5_6]"
            answer="≡/+"
            tests={&["[6_9_1_2 3_0_0_1 2_3_4_5]", "[2_2_2_2_5]"]}
            hidden="↯2_3_4_5⇡120"/>

        <Challenge
            number=3
            prompt="wraps a string in brackets"
            example="\"Hello, World!\""
            answer="$\"[_]\""
            tests={&["\"Uiua\"", "\"🙃\""]}
            hidden="\"tomorrow\""/>
    }
}

#[component]
fn TutorialAdvancedStack() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Advanced Stack Manipulation - Uiua Docs"/>
        <h1>"Advanced Stack Manipulation"</h1>
        <p>"Uiua does not have local variables. With only "<Prim prim=Dup/>", "<Prim prim=Flip/>", and "<Prim prim=Over/>", how do you work with more than 2 values at a time?"</p>

        <Hd id="fork"><Prim prim=Fork/></Hd>
        <p><Prim prim=Fork/>" is a dyadic modifier that takes 2 functions and calls them both on the same set of arguments. The number of arguments used is the maximum of the two functions."</p>
        <Editor example="[⊃+× 3 5]"/>
        <p>"If one of the functions takes more arguments than the other, the function with fewer arguments uses the top-most values."</p>
        <Editor example="⊃×⇌ [1 2 3] 10"/>
        <p>"What's powerful about "<Prim prim=Fork/>" is that it can be chained to use as many functions as you want."</p>
        <Editor example="[⊃⊃⊃+-×÷ 5 8]"/>
        <p><Prim prim=Fork/>" is also good because it does not require that its values be in an array together, so they can be different shapes or types."</p>
        <Editor example="⊃+- 1 @b"/>
        <Editor example="⊃⊃⊃↻↙↘⊡ 2 [1 2 3 4 5]"/>
        <p>"We'll see just how important "<Prim prim=Fork/>" is later in this section."</p>

        <Hd id="both"><Prim prim=Both/></Hd>
        <p><Prim prim=Both/>" is a monadic modifier and a sort of complement to "<Prim prim=Fork/>". While "<Prim prim=Fork/>" calls multiple functions on the same set of arguments, "<Prim prim=Both/>" calls a "<em>"single"</em>" function on "<em>"multiple"</em>" sets of arguments."</p>
        <Editor example="∩⇌ [1 2 3] [4 5 6]"/>
        <p>"Chaining "<Prim prim=Both/>" doubles the number of arguments each time."</p>
        <Editor example="∩∩⇌ [1 2 3] [4 5 6] [7 8 9] [10 11 12]"/>

        <Hd id="bracket"><Prim prim=Bracket/></Hd>
        <p>"To round off the trio, we have "<Prim prim=Bracket/>", which is a dyadic modifier that calls each of its functions on a different set of arguments."</p>
        <Editor example="[⊓+× 1 2 3 4]"/>
        <p><Prim prim=Bracket/>" too can be chained. Each additional function is called on arguments deeper in the stack."</p>
        <Editor example="[⊓⊓⊓+¯×. 1 2 3 4 5 6]"/>

        <Hd id="function-packs">"Function Packs"</Hd>
        <p>"All dyadic modifiers allow a special notation with a single set of "<code>"()"</code>"s with a "<code>"|"</code>" in the middle separating the functions. This is called a "<em>"function pack"</em>"."</p>
        <Editor example="⊓(+|×) 1 2 3 4"/>
        <p>"While all dyadic modifiers can use function packs, "<Prim prim=Fork/>" and "<Prim prim=Bracket/>" allow more than 2 functions to be used. This can sometimes be shorter and/or more readable than chaining the modifier."</p>
        <Editor example="[⊃(+|-|×|÷) 5 8]"/>
        <Editor example="[⊓(+1|×|÷2) 5 10 12 22]"/>

        <Hd id="dip-gap"><Prim prim=Dip/>" and "<Prim prim=Gap/></Hd>
        <p>"The "<Prim prim=Dip/>" modifier temporarily pops the top value on the stack, calls its function, then pushes the value back."</p>
        <Editor example="[⊙+ 1 2 3]"/>
        <p><Prim prim=Dip/>" can be chained to dig deeper into the stack, though try not to dig "<em>"too"</em>" deep, as it makes code harder to read."</p>
        <Editor example="[⊙⊙⊙⊙⊙⊙+ 1 2 3 4 5 6 7 8]"/>
        <p>"One use of "<Prim prim=Dip/>" is to collect values from the stack into an array. Here, a chain of "<Prim prim=Dip/>"s are terminated with "<Prim prim=Identity/>"."</p>
        <Editor example="[⊙⊙⊙∘] 1 2 3 4 5"/>
        <Editor example="{⊙⊙∘} 1 2_3 \"wow\""/>
        <p><Prim prim=Gap/>" "<em>"discards"</em>" the top value on the stack and calls its function."</p>
        <Editor example="⋅+ 1 2 3"/>
        <p>"But wait, "<Prim prim=Pop/>" exists! Why would you need this?"</p>

        <Hd id="planet-notation">"🌍 Planet Notation 🪐"</Hd>
        <p>"The main reason for "<Prim prim=Dip/>" and "<Prim prim=Gap/>" to exist is to be chained with "<Prim prim=Identity/>", often inside of "<Prim prim=Fork/>". They act as a sort of boolean selector to choose which arguments to keep and which to discard in a branch."</p>
        <p>"This is called "<em>"planet notation"</em>" because it looks like the planets in a solar system chart."</p>
        <p>"For example, let's say you want to "<Prim prim=Mul/>" the 2nd and 4th arguments on the stack and discard the rest:"</p>
        <Editor example="×⋅⊙⋅∘ 1 2 3 4"/>
        <p>"Notice how the circles correspond to the stack arguments we want."</p>
        <p>"Maybe you want to "<Prim prim=Add/>" 3 numbers but keep the second 2 on the stack:"</p>
        <Editor example="[⊃⋅⊙∘(++)] 2 5 10"/>
        <p>"You can read "<Prim prim=Gap glyph_only=true/><Prim prim=Dip glyph_only=true/><Prim prim=Identity glyph_only=true/>" as \"discard argument 1, keep argument 2, keep argument 3.\""</p>
        <p>"If you only wanted to keep argument 2, you simply make the expression shorter:"</p>
        <Editor example="[⊃⋅∘(++)] 2 5 10"/>
        <p>"For a more useful example, let's do a complex mathematical expression. We will implement this function (shown here in mathematical notation):"</p>
        <code class="code-block">"f(a,b,c,x) = (a+x)(bx-c)"</code>
        <p>"We'll start with the "<code>"(a + x)"</code>" part. We can grab "<code>"a"</code>" and "<code>"x"</code>" with "<Prim prim=Dip/>" and "<Prim prim=Identity/>", and ignore "<code>"b"</code>" and "<code>"c"</code>" with "<Prim prim=Gap/>"."</p>
        <Editor example="+⊙⋅⋅∘ 1 2 3 4"/>
        <p>"Next, we'll do the "<code>"(bx-c)"</code>" part. We can grab each term with "<Prim prim=Fork/>"."</p>
        <Editor example="-⊃(⋅⋅∘)(×⋅⊙⋅∘) 1 2 3 4"/>
        <p>"The first pair of "<code>"()"</code>"s is not actually necessary, so let's remove them."</p>
        <Editor example="-⊃⋅⋅∘(×⋅⊙⋅∘) 1 2 3 4"/>
        <p>"Finally, we can combine the two parts with another "<Prim prim=Fork/>"."</p>
        <Editor example="×⊃(+⊙⋅⋅∘)(-⊃⋅⋅∘(×⋅⊙⋅∘)) 1 2 3 4"/>
        <p>"If you like, you can factor out the "<Prim prim=Gap/>" in the second part"</p>
        <Editor example="×⊃(+⊙⋅⋅∘)⋅(-⊃⋅∘(×⊙⋅∘)) 1 2 3 4"/>
        <p>"Alternatively, you can use a function pack."</p>
        <Editor example="×⊃(+⊙⋅⋅∘|-⊃⋅⋅∘(×⋅⊙⋅∘)) 1 2 3 4"/>
        <p>"And there you have it! A readable syntax juggling lots of values without any names!"</p>
        <p>"It's annoying to write long lists of names like "<code>"gapdipgapgapide"</code>", so those three functions (plus "<Prim prim=Pop/>") have a special rule in the parser that allows you to write them with only 1 character as long as there are at least 2 characters in the sequence. Also, 'i' and 'p' for "<Prim prim=Identity/>" and "<Prim prim=Pop/>" only work if they are the last character."</p>
        <p>"Try it out!"</p>
        <Editor example="+gdggi 1 2 3 4 5"/>
        <Editor example="+dggdp 1 2 3 4 5"/>
        <p>"In general, planet notation as complex as the mathematical function example above should only be used when it is necessary. For examples like that with 4+ values, it is. However, when working with fewer values, you can get very far with just "<Prim prim=Dup/>" and "<Prim prim=Flip/>". Maybe sprinkle some "<Prim prim=Over/>"s and "<Prim prim=Dip/>"s in there too."</p>

        <Hd id="on-and-by"><Prim prim=On/>" and "<Prim prim=By/></Hd>
        <p>"As you write more Uiua code, you'll find that there is a kind of pattern you'll encounter over and over again. It involves calling a function, then calling another function that re-uses an argument to the first function."</p>
        <p>"One simple example is getting "<code>"n"</code>" numbers between "<code>"0"</code>" and "<code>"1"</code>". One way you may think to solve this is with "<Prim prim=Dup/>" and "<Prim prim=Flip/>"."</p>
        <Editor example="÷:⇡. 5"/> // Should fail
        <p>"This solution works, but as the style diagnostic suggests, it is not quite idiomatic."</p>
        <p>"When the first function you call is dyadic, it can get a little trickier. For example, if you wanted to get all the integers between two numbers, you may try either of the following:"</p>
        <Editor example="+⇡-,: 3 8\n+⊃∘(⇡-) 3 8"/> // Should fail
        <p>"Again, as the style diagnostics tell you, there is a better way."</p>
        <p>"The "<Prim prim=On/>" modifier calls a function but keeps its first argument on top of the stack. This can be used in both of the above examples."</p>
        <Editor example="÷⟜⇡ 5"/>
        <Editor example="+⟜(⇡-) 3 8"/>
        <p>"Having a single glyph for something that can be written as simply "<Prims prims=[Fork, Identity]/>" may seem unnecessary, but you'll find that because the pattern is so common, it makes code easier to both read and write."</p>
        <p>"The "<Prim prim=By/>" modifier is similar. Instead of keeping the first argument on top of the stack, it keeps the last argument below the function's outputs."</p>
        <Editor example="÷⊸⧻ [1 2 3 4]"/>
        <Editor example="▽⊸> 5 [1 8 4 9 2 8 4]"/>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="moves the 4th value on the stack to the top"
            example="1 2 3 4"
            answer="⊃⋅⋅⋅∘⊙⊙∘"
            tests={&["@x [1 2 3] □5 27"]}
            hidden="3 3 3 4"/>

        <Challenge
            number=2
            prompt="adds the second argument to the third and divides by the first"
            example="2 3 5"
            answer="÷⊙+"
            tests={&["1 2 3", "5 10 15"]}
            hidden="2 3 4"/>

        <Challenge
            number=3
            prompt="finds both the sum and product of three arguments"
            example="4 5 6"
            answer="⊃(++|××)"
            tests={&["10 10 10", "1_2 3_4 5"]}
            hidden="[1 2 3] 4 [5 6 7]"/>

        <Challenge
            number=4
            prompt="for numbers A, B, C, and D calculates (A+C)×(B+D)"
            example="1 2 3 4"
            answer="×⊃(+⊙⋅∘|+⋅⊙⋅∘)"
            best_answer="×∩+⊙:"
            tests={&["10 ¯3 1 0", "3 ¯7 2 2"]}
            hidden="1_2 3_4 5_6 7"/>
    }
}

#[component]
fn TutorialInverses() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Inverses - Uiua Docs"/>
        <h1>"Inverses"</h1>
        <p>"Uiua has two modifiers, "<Prim prim=Un/>" and "<Prim prim=Under/>", which work with "<em>"inverses"</em>". The inverse of a function is a function that conceptually \"undoes\" it."</p>
        <p>"Working with inverses is a fundamental part of writing Uiua code. It is an elegant mechanism that captures many different patterns."</p>

        <Hd id="un"><Prim prim=Un/></Hd>
        <p>"The "<Prim prim=Un/>" modifier inverts the behavior of a function."</p>
        <Editor example="°(+1) 5"/>
        <Editor example="°⊟ [1 2]"/>
        <Editor example="°∿ 1"/>
        <p>"As discussed "<A href="/tutorial/arrays#array-model">"previously"</A>", "<Prim prim=Un/><Prim prim=Box/>" removes an array from a box."</p>
        <Editor example=r#"°□ ⊢{"unbox" "me!"}"#/>
        <p>"One interesting use of "<Prim prim=Un/>" is to put an array's rows onto the stack by "<Prim prim=Un/>"ing stack array notation with "<Prim prim=Dip/>" and "<Prim prim=Identity/>". The number of rows in the array must match though!"</p>
        <Editor example="[⊙⊙∘] 1 2 3"/>
        <Editor example="°[⊙⊙∘] [1 2 3]"/>
        <Editor example="°[⊙⊙⊙∘] [1 2 3]"/> // Should fail
        <p><Prim prim=Un/>"ing box array notation will unbox the items."</p>
        <Editor example="°[⊙⊙∘] {1 2_3 \"hmmm\"}"/>
        <Editor example="°{⊙⊙∘} {1 2_3 \"hmmm\"}"/>
        <p>"You can find more uses of "<Prim prim=Un/>" in it's documentation, including a list of all "<Prim prim=Un/>"-compatible functions and modifiers."</p>

        <Hd id="under"><Prim prim=Under/></Hd>
        <p><Prim prim=Under/>" expresses a more powerful inversion pattern. It captures the pattern of doing some transformation, modifying the data, then undoing the transformation."</p>
        <p>"This may not seem immediately useful, but you'll find it is a pattern you encounter everywhere, even in your everyday life. You might open a drawer, take something out, then close the drawer. You might get on a bus, the bus travels, then you get off the bus."</p>
        <p><Prim prim=Under/>" takes two functions which we will call "<code>"F"</code>" and "<code>"G"</code>". It calls "<code>"F"</code>", then calls "<code>"G"</code>", then calls an inverse of "<code>"F"</code>"."</p>
        <p>"Many functions that do not work with "<Prim prim=Un/>" work with "<Prim prim=Under/>" because "<Prim prim=Under/>" can keep track of "<em>"context"</em>". One example of this in action is "<Prim prim=Under/><Prim prim=Pick/>", which allows us to modify an element or row of an array."</p>
        <Editor example="⍜(⊡2|×10) [1 2 3 4]"/>
        <p>"This code picks out item "<code>"2"</code>" of the array, multiplies it by "<code>"10"</code>", then puts it back in the array."</p>
        <p>"If the values passed to "<Prim prim=Under/>"'s functions are not constants, they can also be put outside, albeit in a different order."</p>
        <Editor example="⍜⊡× 2 [1 2 3 4] 10"/>
        <p>"This works because "<Prim prim=Under/>" keeps track of the original array and passes it to the inversion of "<Prim prim=Pick/>"."</p>
        <p>"If you wanted to set a value in an array rather than modifying it, you could use "<Prim prim=Pop/>" or "<Prim prim=Gap/>" instead of "<Prim prim=Mul/>"."</p>
        <Editor example="⍜(⊡2)⋅∞ [1 2 3 4]\n⍜⊡◌ 2 [1 2 3 4] ∞"/>
        <p>"It's not just "<Prim prim=Pick/>"! Many functions work with "<Prim prim=Under/>"!"</p>
        <Editor example="⍜(↙2)/× [3 5 4 2]"/>
        <Editor example="⍜(↻3|⊂0) [1 2 3 4 5]"/>
        <Editor example="⍜×⁅ 1e3 π"/>
        <Editor example=".↯3_4⇡12\n⍜♭⇌"/>
        <p>"You can even use "<Prim prim=Under/>" on a function that has already been "<Prim prim=Un/>"ed. This is a nice way to work with "<Prim prim=Box/>"ed data."</p>
        <Editor example=r#"≡⍜°□(⊂:@!) {"wow" "cool" "omg"}"#/>
        <br/>
        <p>"Let's say you wanted to utilize a struct-like pattern. Uiua does not have structs or objects with fields like many other languages do, but you can simulate them with box arrays. This can be slow, so you should not do this with any data that needs to accessed in tight loops."</p>
        <p><Prim prim=Under/>" allows a field getter to also be a setter!"</p>
        <Editor example=r#"Person ← {⊙⊙∘}
Name ← °□⊡0
Surname ← °□⊡1
Age ← °□⊡2

FmtPerson ← $"_ is _ years old" ⊃(Name|Age)
PassYear ← ⍜Age(+1)

Dan ← Person "Dan" "Danson" 31
FmtPerson Dan
FmtPerson PassYear Dan"#/>
        <p>"You can find more uses of "<Prim prim=Under/>" in it's documentation, including a list of all "<Prim prim=Under/>"-compatible functions and modifiers."</p>

        <Hd id="setting-inverse">"Setting Inverses"</Hd>
        <p>"Many functions, especially more complex ones, do not have well-defined inverses. However, you can use the "<Prim prim=SetInverse/>" and "<Prim prim=SetUnder/>" modifiers to define them yourself."</p>
        <p><Prim prim=SetInverse/>" sets a simple inverse that is compatible with "<Prim prim=Un/>"."</p>
        <p>"For example, "<Prim prim=First/>" does not have an "<Prim prim=Un/>"-compatible inverse, but we can define one."</p>
        <Editor example="MyFirst ← setinv(⊢|[∘])\nMyFirst [1 2 3]\n°MyFirst 5"/>
        <p>"This inverse is also compatible with "<Prim prim=Under/>"."</p>
        <Editor example="MyFirst ← setinv⊢[∘]\n⍜⊢(×10) [2 3 4]\n⍜MyFirst(×10) [2 3 4]"/>
        <p>"Inverses should have the opposite signature of function they are the inverse of. If it does not, you will get a warning."</p>
        <Editor example="F = setinv+-"/> // Should fail
        <p><Prim prim=SetUnder/>" is more complicated. See its documentation for how to use it."</p>
        <p><Prim prim=SetInverse/>" and "<Prim prim=SetUnder/>" can be nested so that an inverse can be fully defined in all cases."</p>
        <p>"This example shows how the different inverses get called."</p>
        <Editor example=r#"F ← setund(setinv("normal"|"inverse")|"do"|"undo")
F
°F
{⍜F"G"}"#/>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="adds 100 to the 2nd and 4th rows of an array"
            example="[1 2 3 4 5]"
            answer="⍜(⊏1_3|+100)"
            tests={&["↯4_3⇡12", "⊚10"]}
            hidden="↯4_3_4⇡24"/>

        <Challenge
            number=2
            prompt="transposes an array so that the last axis becomes the first"
            example="[[1_2_3 4_5_6] [7_8_9 10_11_12]]"
            answer="°⍉"
            tests={&["↯2_4⇡8", "↯2_2_4⇡16"]}
            hidden="↯2_2_2_2⇡16"/>

        <Challenge
            number=3
            prompt="multiplies the first column of a matrix by 10"
            example="[1_2 3_4]"
            answer="≡⍜⊢(×10)"
            best_answer="⍜(⊢⍉|×10)"
            tests={&["+1↯3_3⇡9", "↯2_4 1"]}
            hidden="[1_2]"/>
    }
}

#[component]
fn TutorialControlFlow() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Control Flow - Uiua Docs"/>
        <h1>"Control Flow"</h1>
        <p>"Uiua, and array languages in general, require much less control flow than other programming languages. Most operations that would be loops in other languages are simply operations on arrays. Because boolean operations return numbers, a lot of checks that would be done with "<code>"if"</code>" statements in other languages become mathematical or indexing operations in array languages."</p>
        <p>"For example, if you wanted to split an array of numbers into an array of odds and an array of evens, you might do it like this in a language like Python:"</p>
        <code class="code-block">"\
def splitArray(array):
    even = []
    odd = []
    for i in array:
        if i % 2 == 0:
            even.append(i)
        else:
            odd.append(i)
    return even, odd

splitArray([1, 2, 3, 7, 2, 4, 5])"</code>
        <p>"In Uiua, it is much simpler, and there are no "<code>"if"</code>"s or "<code>"for"</code>"s to be found:"</p>
        <Editor example="F ← ∩▽¬,,◿2.\nF [1 2 3 7 2 4 5]"/>
        <p>"That being said, not every problem lends itself to array operations. Uiua has a few methods for handling such cases."</p>

        <Hd id="repeat-do">"Looping with "<Prim prim=Repeat/>" and "<Prim prim=Do/></Hd>
        <p>"The "<Prim prim=Repeat/>" modifier takes a function and a number and calls the function that many times."</p>
        <p>"Here, we "<Prim prim=Mul/>" a number by "<code>"2"</code>", "<code>"10"</code>" times."</p>
        <Editor example="⍥(×2)10 5"/>
        <Editor example="⍥/+2 ↯3_3⇡9"/>
        <p><Prim prim=Repeat/><Prim prim=Rand/>" inside "<code>"[]"</code>"s is a common pattern for generating a list of random numbers."</p>
        <Editor example="⁅[⍥⚂]5"/>
        <p><Prim prim=Repeat/>" is also useful for conditionally calling a function. Because booleans in Uiua are just numbers, "<Prim prim=Repeat/>"ing with a boolean value will call a function "<code>"0"</code>" or "<code>"1"</code>" times."</p>
        <Editor example="F ← ⍥(×10)<10.\nF 5\nF 12"/>
        <p><Prim prim=Repeat/>"'s glyph is a combination of a circle, representing a loop, and the 𝄇 symbol from musical notation."</p>
        <p>"The "<Prim prim=Do/>" modifier takes a loop function and a condition function. It repeatedly calls the loop function as long as the condition function returns "<code>"1"</code>"."</p>
        <Editor example="⍢(×2|<1000) 1"/>
        <Editor example="◌⍢(⊃(×2)⊂|<100) 1 []"/>
        <p>"While "<Prim prim=Do/>" is very powerful, it should only be used when necessary."</p>
        <p><Prim prim=Do/>" is the only way to do an infinite loop in Uiua. To do so, simply use "<code>"1"</code>" as the condition function."</p>

        <Hd id="try">"Catching errors with "<Prim prim=Try/></Hd>
        <p>"The "<Prim prim=Try/>" modifier takes two functions. If the first function throws an error, the second function is called to handle it."</p>
        <p>"The function must have the same number of outputs."</p>
        <p>"The handler function can take at most 1 more agument that the first function."</p>
        <p>"We can see how this works by using it with "<Prim prim=Parse/>"."</p>
        <p>"If the handler function has 0 arguments, then it is simply called. This is a nice way to provide default values in the event of a failure."</p>
        <Editor example="⍣⋕0 \"5\"\n⍣⋕0 \"dog\""/>
        <p>"If the handler function has 1 argument, then the original argument is passed to it."</p>
        <Editor example="⍣⋕∘ \"5\"\n⍣⋕∘ \"dog\""/>
        <p>"If the handler function takes 1 more argument than the first function, then the error is also passed to it."</p>
        <Editor example="⍣⋕{⊙∘} \"5\"\n⍣⋕{⊙∘} \"dog\""/>
        <p>"You can read about more uses of "<Prim prim=Try/>" in its documentation."</p>

        <Hd id="switch"><Prim prim=Switch/></Hd>
        <p>"The "<Prim prim=Switch/>" modifier uses a selector to choose one of its functions to call."</p>
        <Editor example="⨬(3)(5) 0\n⨬(3)(5) 1"/>
        <p>"The selector goes above the arguments on the stack."</p>
        <p>"Here, we "<Prim prim=Add/>" if the selector is "<code>"0"</code>" and "<Prim prim=Sub/>" if the selector is "<code>"1"</code>"."</p>
        <Editor example="⨬+- 0 3 5\n⨬+- 1 3 5"/>
        <p>"Non-scalar selectors are allowed. They allow a different function to be evaluated for each row of the input array(s)."</p>
        <Editor example="⨬+- [1 0 1] [1 2 3] [4 5 6]"/>
        <p><Prim prim=Switch/>" can use a "<A href="/tutorial/advancedstack#function-packs">"function pack"</A>" to select from more functions."</p>
        <Editor example="⨬(+|-|×|÷) [1 2 0 3] [...2] [...5]"/>
        <Editor example="⨬(×10|+1|⨬¯∘ =2.) ◿3. [2 9 4 0 8 3]"/>
        <p>"With "<Prim prim=IndexOf/>", "<Prim prim=Switch/>" can be used to implement behavior similar to "<code>"switch"</code>" statements in other languages."</p>
        <Editor example="F ← (\n  ⊗□:{\"foo\" \"bar\" \"baz\"}\n  ⨬(+1|×10|÷2|¯)\n)\nF \"foo\" 5\nF \"bar\" 5\nF \"baz\" 5\nF \"wow\" 5"/>
        <p>"Each branch can have a signature specified. For the overall "<Prim prim=Switch/>" to have a valid signature, all branches must either change the height of the stack by the same amount "<em>"or"</em>" return the same number of outputs."</p>
        <Editor example="F ← ⨬(|2 ×||3.2 ⊃(++)×)\n[F 0 2 3 4]\n[F 1 2 3 4]"/>
        <p>"Signatures in "<Prim prim=Switch/>" functions are a bit messy, so try to avoid them when possible."</p>
        <p>"Because a second "<code>"|"</code>" immediately after another indicates a signature, branches that do nothing must contain "<Prim prim=Identity/>"."</p>
        <Editor example="F ← ⨬(+5|∘|÷10)+∩>5,10.\n[F2 F6 F200]\nF[2 6 200]"/>

        <Hd id="recursion">"Recursion"</Hd>
        <p>"A bound function that refers to its own name is a "<a href="https://en.wikipedia.org/wiki/Recursion_(computer_science)">"recursive function"</a>". A function that calls itself can easily recurse infinitely, so it is important to have a "<em>"base case"</em>" that stops the recursion when a condition is met. Switch functions are great for this."</p>
        <p>"As a simple example, here is a function that calculates the factorial of a number. Note that you should not actually do this, as "<Prims prims=[Reduce, Mul, Add]/><code>"1"</code><Prims prims=[Range]/>" is shorter, faster, and more idiomatic."</p>
        <Editor example="Fact ← |1 ⨬(×Fact-1.|1)<2.\nFact 5"/>
        <p>"The base case is when the input is "<code>"1"</code>". In this case, the function returns "<code>"1"</code>". Otherwise, it multiplies the input by the result of calling itself with the input decremented by "<code>"1"</code>"."</p>
        <p>"Recursive functions are required to have signatures declared."</p>
        <p>"Recursion is only recommended if a particular problem "<em>"really"</em>" calls for it. Recursion in Uiua can be slow, and there is a limit to how deep you can recur."</p>
        <p>"It is usually better to use either array-based methods or iteration with "<Prim prim=Repeat/>" or "<Prim prim=Do/>"."</p>

        <Hd id="assert"><Prim prim=Assert/></Hd>
        <p>"The "<Prim prim=Assert/>" function takes any value and a condition. If the condition is anything but "<code>"1"</code>", the value is thrown as an error that can be caught with "<Prim prim=Try/>"."</p>
        <Editor example="F ← ⍣(¯⍤10≤10.)∘\nF 5\nF 12"/>
        <p>"If the "<Prim prim=Assert/>"ed value is never caught, it becomes an error."</p>
        <Editor example="F ← ¯⍤\"too big!\"≤10.\nF 5\nF 12"/> // Should fail
        <p>"Using "<Prim prim=Assert/>" for this purpose will be covered more in the "<A href="/tutorial/testing">"section on testing"</A>"."</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="pushes \"small\" if a number is less than 10, \"medium\" if it is less than 100, and \"large\" otherwise"
            example="17"
            answer=r#"⨬("small"|"medium"|"large")/+≥[10 100]"#
            tests={&["3", "50", "2357"]}
            hidden="10"/>

        <Challenge
            number=2
            prompt="multiplies an array by its reverse until any element is greater than 1000"
            example="[1.5 8 2]"
            answer="⍢(×⇌.|≤1000/↥)"
            tests={&["[1 2 3]", "[¯6 5 1]"]}
            hidden="7"/>
    }
}

#[component]
fn TutorialPatternMatching() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Pattern Matching - Uiua Docs"/>
        <h1>"Pattern Matching"</h1>
        <p>"Uiua has a powerful mechanism for matching patterns in arrays to conditionally extract data."</p>

        <Hd id="un-patterns"><Prim prim=Un/>" Patterns"</Hd>
        <p><Prim prim=Un/>" can be applied to a constant value to form a function that throws an error if the top value on the stack does not match the constant."</p>
        <Editor example="°5 5"/>
        <Editor example="°5 3"/> // Should fail
        <p>"This works for arrays as well."</p>
        <Editor example="°[1 2 3] [1 2 3]"/>
        <Editor example="°[1 2 3] [4 5 6]"/> // Should fail
        <p>"This is not very useful on its own, but it can be composed with other inverses to form more complex patterns."</p>
        <p>"A primary pattern of note is using stack array notation with planet notation to form patterns that match arrays with certain values and extract the others."</p>
        <Editor example="°[1⊙3] [1 2 3]"/>
        <Editor example="°[1⊙3] [4 5 6]"/> // Should fail
        <p>"These can be arbitrarily nested."</p>
        <Editor example="°[1 2⊙⊙(5∘)] [1 2 3 4 5 6]"/>
        <p><Prim prim=Un/><Prim prim=Join/>" with a constant can also be used to match arrays with a certain prefix."</p>
        <Editor example="°(⊂1) [1 2 3]"/>
        <Editor example="°(⊂1) [4 5 6]"/> // Should fail
        <Editor example="°(⊂1_2) [1 2 3]"/>
        <p>"To match a suffix, you can use "<Prim prim=Under/><Prim prim=Reverse/>"."</p>
        <Editor example="⍜⇌°(⊂3) [1 2 3]"/>

        <Hd id="with-try">"Matching multiple patterns with "<Prim prim=Try/></Hd>
        <p>"Single patterns are of limited usefulness on their own. Because they throw errors when matching fails, you can attempt to match additional errors using "<Prim prim=Try/>"."</p>
        <p><Prim prim=Try/>" accepts arbitrarily long function packs, so you can match as many patterns as you want in a simple way."</p>
        <p>"In this example, we run different code depending on which pattern matches."</p>
        <Editor example="F ← ⍣(×10°[1⊙3]|°(⊂5)|⇌)\nF [5 6 7]\nF [1 2 3]\nF \"abc\""/>
        <p>"Having more or longer patterns may be easier to read if each pattern gets its own line."</p>
        <Editor example="F ← ⍣(\n  ×10 °[1⊙3]\n| °(⊂5)\n| ⇌\n)"/>

        <Hd id="format-string-patterns">"Format String Patterns"</Hd>
        <p><Prim prim=Un/>" works with format strings to extract substrings where the "<code>"_"</code>"s are. While the "<Prim prim=Regex/>" function is available, it is often more complex than is necessary. In these cases, format string patterns are more appropriate."</p>
        <Editor example="°$\"_, _, _\" \"1, 2, 3\""/>
        <Editor example="°$\"_, _, _\" \"1, 2, 3, 4, 5\""/>
        <Editor example="°$\"Hello, _!\" \"Hello, World!\""/>
        <p>"More precisely, format string patterns form a regex that replaces all "<code>"_"</code>"s from the format string with "<code>"(.+?|.*)"</code>", where "<code>"."</code>" also matches newlines."</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt=view!("removes a leading "<code>"0"</code>" from an array of numbers or prepends a "<code>"0"</code>" if it is missing")
            example="[0 1 2 3]"
            answer="⍣°(⊂0)(⊂0)"
            tests={&["[4 0 9]", "[0 0 3 4]"]}
            hidden="[0 0 0 0]"/>

        <Challenge
            number=2
            prompt=view!("splits a string on the first "<code>"-"</code>" and returns the two parts")
            example="\"hello-world\""
            answer="°$\"_-_\""
            tests={&["\"foo-bar\"", "\"1-2-3\""]}
            hidden="\"-\""/>

        <Challenge
            number=3
            prompt=view!("matches the argument against string prefixes "<code>"a"</code>", "<code>"bc"</code>", or "<code>"def"</code>" and adds "<code>"1"</code>", "<code>"2"</code>", or "<code>"3"</code>" to the second argument respectively, or adds "<code>"10"</code>" otherwise")
            example="\"definate\" 5"
            answer="⍣(+1 ◌°$\"a_\"|+2 ◌°$\"bc_\"|+3 ◌°$\"def_\"|+10 ◌)"
            tests={&["\"abc\" 1", "\"bcause\" [1 2 3]"]}
            hidden="\"wow\" 4"/>
    }
}

#[component]
fn TutorialAdvancedArray() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Advanced Array Manipulation - Uiua Docs"/>
        <h1>"Advanced Array Manipulation"</h1>
        <p>"Sometimes the operation you need to perform on an array is more complicated than modifiers like "<Prim prim=Reduce/>", "<Prim prim=Rows/>", or "<Prim prim=Table/>" allow."</p>

        <Hd id="fix"><Prim prim=Fix/></Hd>
        <p><Prim prim=Rows/>" can be used to iterate over multiple arrays. The nth row of each array will be passed to the function, and the result will be put in a new array."</p>
        <Editor example="≡⊂ 1_2_3 4_5_6"/>
        <p>"Usually, the arrays must have the same number of rows."</p>
        <Editor example="≡⊂ 1_2_3 4_5"/> // Should fail
        <p>"However, there is an exception for arrays that have exactly one row. In this case, that row will be repeated for each row of the other array(s)."</p>
        <Editor example="≡⊂ 1_2_3 4"/>
        <Editor example="≡⊂ 1 2_3_4"/>
        <Editor example="≡(⊂⊂) 1 2_3_4 5"/>
        <p>"Notice that the second argument here is a 2D array with 1 row of 2 elements. It will be repeated just like the scalars above."</p>
        <Editor example="≡⊟ [1_2 3_4 5_6] [¯1_0]"/>
        <p>"If we want to combine each row of one array with copies of another, we can turn one of the arrays into a single row array with "<Prim prim=Fix/>". "<Prim prim=Fix/>" adds a 1 to the front of the shape of an array."</p>
        <Editor example="¤.1_2_3"/>
        <p>"Here, we "<Prim prim=Fix/>" "<code>"1_2_3"</code>" so that it is reused for each row of "<code>"4_5_6"</code>"."</p>
        <Editor example="≡⊂ ¤ 1_2_3 4_5_6"/>
        <p>"If we have a bunch of arrays and want to choose which ones are fixed and which are not, we can use planet notation."</p>
        <Editor example="≡⊂ ⊙¤ 1_2_3 4_5_6"/>
        <Editor example="≡(⊂⊂⊂) ⊓⊓⊓∘¤¤∘ 1_2_3 4_5_6 7_8_9 10_11_12"/>
        <Editor example="≡(⊂⊂⊂) ⊙∩¤     1_2_3 4_5_6 7_8_9 10_11_12"/>
        <p><Prim prim=Fix/>" also works without "<Prim prim=Rows/>" with pervasive dyadic functions."</p>
        <Editor example="-  [1 2 3]  [4 5 6]\n- ¤[1 2 3]  [4 5 6]\n-  [1 2 3] ¤[4 5 6]"/>
        <Editor example="-  1_3 [3_4 5_6 7_8]"/> // Should fail
        <Editor example="- ¤1_3 [3_4 5_6 7_8]"/>

        <Hd id="rerank"><Prim prim=Rerank/></Hd>
        <p>"The above examples dig into an array from the top down. But what if you want to think about the array from the "<em>"bottom up"</em>"?"</p>
        <p>"The "<Prim prim=Rerank/>" function changes the rows of an array to have the specified rank."</p>
        <Editor example="☇3 ↯2_2_2_5⇡40 # The rows are already rank 3"/>
        <Editor example="☇2 ↯2_2_2_5⇡40"/>
        <Editor example="☇1 ↯2_2_2_5⇡40"/>
        <Editor example="☇0 ↯2_2_2_5⇡40 # Equivalent to ♭ deshape"/>
        <p>"You can then use "<Prim prim=Rows/>" to iterate over arrays of that rank."</p>
        <Editor example="≡□ ☇1 ↯2_2_2_3⇡24"/>
        <p>"You can think of "<Prim prim=Rerank/>" as combining the dimensions of the array that are above the specified rank into a single dimension."</p>
        <p>"In the example above, the shape information of the original array is lost."</p>
        <p>"If you want to keep the part of the shape that is above the specified rank, you can use "<Prim prim=Under/>". This will "<em>"uncombine"</em>" the combined dimensions after the operation is complete."</p>
        <Editor example="⍜(☇1)≡□ ↯2_2_2_3⇡24"/>
        <p>"Notice in the above example that we specified the rank "<code>"1"</code>", and it was rank "<code>"1"</code>" arrays that were "<Prim prim=Box/>"ed."</p>
        <p>"We can think of this pattern as allowing us to operate on arrays of the specified rank within the greater array. For example, by changing the rank to "<code>"2"</code>", we end up "<Prim prim=Box/>"ing matrices instead of lists."</p>
        <Editor example="⍜(☇2)≡□ ↯2_2_2_3⇡24"/>
        <p>"The specified rank can still be dynamic in this case by simply putting it on the stack."</p>
        <Editor example="⍜☇≡□ 1 ↯2_2_2_3⇡24"/>
        <Editor example="⍜☇≡□ 2 ↯2_2_2_3⇡24"/>
        <p>"You can use "<Prim prim=Under/><Prim prim=Both/><Prim prim=Rerank/>" to "<Prim prim=Rerank/>" 2 arrays. Here, we insert one of the ranks for "<Prim prim=Rerank/>" using "<Prim prim=Dip/>"."</p>
        <Editor example="⍜∩☇≡⊂ 1⊙1 ↯6_2⇡12 ↯2_3_4⇡24"/>

        <Hd id="table">"Multi-dimensional "<Prim prim=Table/></Hd>
        <p><Prim prim=Table/>" called on arrays of multiple dimensions calls its function on all "<em>"combinations"</em>" of rows of its arguments."</p>
        <Editor example="⊞⊂ η_π_τ ↯3_3⇡9"/>
        <p><Prim prim=Table/>" can be useful when working with "<Prim prim=Rerank/>"ed or "<Prim prim=Fix/>"ed arrays."</p>
        <p>"In this example, we apply a table of rotations to each matrix cell of a 3D array."</p>
        <Editor example="°⊚ ≡[..]⇡3           # Target array
[0_0 0_1]_[¯1_1 1_0] # Rotations table
,,                   # Copy to see inputs
⍜(☇1)⊞↻              # All rotation combinations
≡≡□                  # Box for display"/>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="adds the first argument list to each row of the second argument matrix"
            example="1_2_3 [4_5_6 7_8_9]"
            answer="+¤"
            tests={&["10_20 ↯4_2⇡8", "\"Wow\" ¯[10_0_10 19_14_19]"]}
            hidden="1_2 [3_4]"/>

        <Challenge
            number=2
            prompt="rotates all rank 2 arrays in the second argument by all rank 1 arrays in the first"
            example="[¯1_¯2 0_1] [.] ↯3_4⇡12"
            answer="⊞↻ ☇1⊙(☇2)"
            best_answer="⊞↻ ∩☇1⊙2"
            tests={&["[0_2 2_1 1_1] ⊞×⊞×.↘1.+1⇡3"]}
            hidden="1 [1 2 3]"/>
    }
}

#[component]
fn TutorialThinkingWithArrays() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Thinking with Arrays - Uiua Docs"/>
        <h1>"Thinking with Arrays"</h1>
        <p>"So far, we've covered the mechanics of working with arrays in Uiua. However, if you are new to the array paradigm, it may not be clear how to use arrays to solve problems."</p>
        <p>"This section covers some of the common functions and modifiers that pop up when solving many different problems."</p>

        <Hd id="masks-and-keep">"Masks and "<Prim prim=Keep/></Hd>
        <p>"Many languages have some sort of "<code>"filter"</code>" function that takes a predicate and a list and returns a list of all the elements that satisfy the predicate. In array languages, we take a different approach."</p>
        <p>"First, we create a "<em>"mask"</em>" array. A mask array is an array of "<code>"0"</code>"s and "<code>"1"</code>"s where "<code>"1"</code>"s represent the rows that satisfy the predicate. For pervasive functions, this is extremely simple."</p>
        <p>"For example, if we wanted to create a mask of all numbers greater that 4, we simply treat the whole array as a single unit."</p>
        <Editor example=">4. [2 8 3 9 1 7 2]"/>
        <p>"The "<Prim prim=Keep/>" function takes a mask array and an array and returns an array of all the rows that have a "<code>"1"</code>" in the mask. This is essentially a filter."</p>
        <Editor example="▽ >4. [2 8 3 9 1 7 2]"/>
        <p><Prim prim=Keep/>" also works with "<Prim prim=Under/>" so that you can modify the rows that have a "<code>"1"</code>" in the mask."</p>
        <Editor example="⍜▽(×10) >4. [2 8 3 9 1 7 2]"/>
        <p><Prim prim=Keep/>" has a few other use cases with non-masks. See its documentation for more."</p>

        <Hd id="where"><Prim prim=Where/></Hd>
        <p>"The "<Prim prim=Where/>" function converts a mask array into an array of indices where the mask is "<code>"1"</code>"."</p>
        <Editor example="⊚. >4. [2 8 3 9 1 7 2]"/>
        <p>"This works with multi-dimensional arrays as well."</p>
        <Editor example="⊚. >4. [2_8_3 9_1_7]"/>
        <p><Prim prim=Un/><Prim prim=Where/>" converts an array of indices into a mask array."</p>
        <Editor example="°⊚ [3 9 5 8]"/>
        <Editor example="°⊚ [1_2 3_4]"/>
        <p><Prim prim=Select/><Prim prim=Where/>" is equivalent to "<Prim prim=Keep/>" (at least for boolean predicates)."</p>
        <Editor example="⊏⊚ =0◿2. [2 8 3 9 1 7 2]"/>
        <Editor example="▽  =0◿2. [2 8 3 9 1 7 2]"/>

        <Hd id="scan"><Prim prim=Scan/></Hd>
        <p>"The "<Prim prim=Scan/>" modifier is similar to "<Prim prim=Reduce/>", but it returns an array of all the intermediate results."</p>
        <Editor example="/+ [1 2 3 4]\n\\+ [1 2 3 4]"/>
        <p>"This can be useful when used on a mask."</p>
        <p>"For example, if we wanted to get the first word of a string, we could start by creating a mask of all the non-space characters."</p>
        <p>"Then we can use "<Prim prim=Scan/><Prim prim=Mul/>" to zero the mask after the first word."</p>
        <p>"Finally, we can use "<Prim prim=Keep/>" to apply the mask and get the first word."</p>
        <p>"Use the arrows to see how the mask changes."</p>
        <Editor example=r#"▽ \× ≠@ . "What's the first word?""#/>

        <Hd id="fill"><Prim prim=Fill/></Hd>
        <p>"Recall that the "<Prim prim=Fill/>" modifier sets a \"fill value\" that can be used by certain functions."</p>
        <p>"One common use is to set a default value that will be used when the shapes of arrays do not match."</p>
        <Editor example="⬚0+ 10_20 3_4_5_6"/>
        <p>"For example, if you wanted to logical OR two masks with different shapes, you could use "<Prim prim=Fill/>" with a different fill value depending on what you want to do with the mismatched parts."</p>
        <Editor example="⬚0↥ 1_0_0_1_0 0_1_0\n⬚1↥ 1_0_0_1_0 0_1_0"/>
        <p>"Another interesting use is a "<Prim prim=Fill/>"ed "<Prim prim=Rotate/>". Instead of wrapping values around, it fills in one side of the array with the fill value."</p>
        <Editor example="  ↻¯2 [1 2 3 4 5]\n⬚0↻¯2 [1 2 3 4 5]"/>

        <Hd id="partition"><Prim prim=Partition/></Hd>
        <p><Prim prim=Partition/>" is a powerful modifier that splits up an array based on a list of consecutive keys. Before explaining it further, let's look at a simple example of a very common use case: splitting a string into words."</p>
        <Editor example=r#"⊜□ ≠@ . "Look at that!""#/>
        <p>"First, we create a mask of all the non-space characters. Then, "<Prim prim=Partition/>" calls "<Prim prim=Box/>" on each section of the string that corresponds to a run of "<code>"1"</code>"s in the mask."</p>
        <p>"Here is another example using "<Prim prim=Partition/><Prim prim=Box/>" with the inputs explicitly defined."</p>
        <Editor example="[1 2 3 4 5 6 7 8]\n[1 1 0 5 6 6 0 1]\n⊜□"/>
        <p>"Notice that "<code>"0"</code>"s in the keys array cause the corresponding sections of the input array to be skipped, so "<code>"3"</code>" and "<code>"7"</code>" are omitted from the output."</p>
        <p>"We use "<Prim prim=Box/>" here because the resulting sections have different lengths. If we expect the sections to have the same lengths, we can use "<Prim prim=Identity/>" instead."</p>
        <Editor example="[1 2 3 4 5 6 7 8]\n[1 1 2 2 0 0 3 3]\n⊜∘"/>
        <p><Prim prim=Partition/>" is very useful when working with strings. See the "<A href="/tutorial/strings">"Strings tutorial"</A>" for more."</p>
        <p>"A hint for one of the challenges below: "<Prim prim=Partition/>" works with "<Prim prim=Under/>"!"</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="negates each number in a list that is not a multiple of 3"
            example="[1 2 3 4 5 6 7 8 9]"
            answer="⍜▽¯ ≠0◿3."
            tests={&["[3 0 1 8]", "[3 6 9 12 15 18 21 25 27]"]}
            hidden="[3 6 9 4]"/>

        <Challenge
            number=2
            prompt="returns the last word of a string"
            example=r#""What's the last word?""#
            answer=r#"▽ ⍜⇌\× ≠@ ."#
            tests={&[r#""Um, I um, arrays""#, r#""I like trains""#]}
            hidden=r#"Wow"#/>

        <Challenge
            number=3
            prompt="for every multiple of 3 in a list, multiplies the following number by 10"
            example="[1 2 3 4 5 6 7]"
            answer="⍜▽(×10) ⬚0↻¯1 =0◿3."
            tests={&["[2 9 3 8 7 1]", "[3 3 3 3]"]}
            hidden="[]"/>

        <Challenge
            number=4
            prompt="given a matrix of 0s an 1s, only keeps the 1s that have even x and y coordinates"
            example="[1_1_0 0_1_1 0_1_1]"
            answer="⍜⊚(▽≡/×¬⊸◿2)"
            tests={&["↯3_4 1_0_1", "↯4_4 1_0_0_1_0"]}
            hidden="[1_1 1_1]"/>

        <Challenge
            number=5
            prompt="reverses each word in a string but keeps the words in the same order"
            example=r#""get in the racecar""#
            answer="⍜⊜□≡⇌ ≠@ ."
            tests={&[r#""arrays are neat""#, r#""wow mom""#]}
            hidden=r#""Wow, mom!""#/>
    }
}

#[component]
fn TutorialMacros() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Macros - Uiua Docs"/>
        <h1>"Macros"</h1>
        <p>"Defining your own functions that work on arrays is pretty easy. Just a name, a "<code>"←"</code>", and you're done."</p>
        <p>"But what if you want to define functions that use other functions?"</p>

        <Hd id="placeholders-and-bangs">"Placeholders and "<code>"!"</code>"s"</Hd>
        <p>"Anywhere you can put a built-in or inline function, you can also put a "<code>"^!"</code>". This is called a "<em>"placeholder"</em>"."</p>
        <p>"Any named function with "<code>"^!"</code>"s in it is a macro."</p>
        <p>"However, there is one additional requirement: macros must have names that end in as many "<code>"!"</code>"s as the number of functions they take."</p>
        <p>"Macros work similarly to modifiers. They take some function arguments and modify how they are used."</p>
        <p>"Lets look at a simple example using "<Prim prim=Reduce/>". It reduces a function over the numbers up to the given range."</p>
        <Editor example="\
ReduceRange! ← /^!+1⇡
ReduceRange!+5
ReduceRange!×4"/>
        <p>"Here is another simple example which calls a function on a reversed version of each row of an array."</p>
        <Editor example="\
OnRev! ← ≡⍜⇌^!
OnRev!(↘1) ↯3_4⇡12
OnRev!(⊂π) ↯3_4⇡12"/>
        <p>"A macro can take as many functions as you want. Modifiers with two or more function arguments will be formatted to use "<code>"‼"</code>"s as needed. Try running the following example to format it."</p>
        <Editor example="\
F!!! ← ⊂/^!⊃^!^!
F!!!+×⊂ [1 2 3][4 5 6]"/>

        <Hd id="operand-functions">"Operand Functions"</Hd>
        <p>"When Uiua code is compiled, a macro passes its operands as values on a special operand stack. This stack can be manipulated in a few ways to make macros more powerful."</p>
        <p><code>"^!"</code>" is actually an operand function. It pops the operand at the top of the operand stack and inlines it into its place in the macro."</p>
        <p>"There are also operand function analogues for "<Prim prim=Dup/>", "<Prim prim=Flip/>", and "<Prim prim=Over/>"."</p>
        <p>"These are "<code>"^."</code>", "<code>"^:"</code>", and "<code>"^,"</code>" respectively."</p>
        <p>"Like normal Uiua code, operand functions are evaluated from right to left."</p>
        <p>"However, operands are initially pushed to the stack in reverse order. This means that the macro operand furthest to the right starts at the top of the operand stack."</p>
        <p>"Let's look at a simple example to see how this ordering works."</p>
        <Editor example="F‼ ← ^!^!\nG‼ ← ^!^!^:\nF‼(⊂1|⊂2) []\nG‼(⊂1|⊂2) []"/>
        <p>"If we wanted to call each of two functions twice, we could use a similar pattern to what we use in normal Uiua code."</p>
        <Editor example="[,, 1 2]\nF‼ ← ^!^!^!^!^,^,\nF‼(⊂1|⊂2) []"/>

        <Hd id="two-kinds">"Two Kinds of Macros"</Hd>
        <p>"The macros described so far are called "<em>"stack macros"</em>", because they move operands around on a stack."</p>
        <p>"But Uiua actually has a second kind of macro. "<em>"Array macros"</em>" put their operands in an array. The array can then be arbitrarily manipulated with normal Uiua code."</p>

        <Hd id="array-macros">"Array Macros"</Hd>
        <p>"Array macros are defined by putting a "<code>"^"</code>" right after the binding's "<code>"←"</code>". Array macro names must still end in some number of "<code>"!"</code>"s."</p>
        <p>"Here is a basic example that simply prints its operands. It returns the number "<code>"5"</code>" as the actual generated code."</p>
        <Editor example="F‼ ←^ \"5\" &pf\nF‼⊂(+1)"/>
        <p>"As you can see, the operands are passed to the function as an array of boxed strings."</p>
        <p>"Array macros may be passed a function pack operand. Each operand from the pack will be put in the array."</p>
        <Editor example="F! ←^ $\"_\"\nF!(+|-|×|÷)"/>
        <p>"The array macro's function must return either a string or an array of boxed strings. This value will be converted back to Uiua code and compiled as normal."</p>
        <p>"Format strings can help a lot in generating new code. For example, if we wanted to make a version of "<Prim prim=Both/>" that calls its function on an arbitrary number of sets of values, we could use "<Prim prim=Reshape/>" and "<Prim prim=Bracket/>"."</p>
        <Editor example="All‼ ←^ $\"⊓(_)\" /$\"_|_\" ↯⋕ °{⊙∘}\n[All‼3+ 1 2 3 4 5 6]"/>
        <p>"First, we extract the two operands: the count and the function. The count comes in as a string, so we have to "<Prim prim=Parse/>" it before using "<Prim prim=Keep/>" to make an array of copies of the function."</p>
        <p>"We use "<Prim prim=Reduce/>" with a format string to form the branches of a function pack, then use another format string to put them in "<Prim prim=Bracket/>"."</p>
        <p>"The resulting string is then compiled as Uiua code."</p>
        <br/>
        <p>"Array macros have the ability to create new bindings, including new macros."</p>
        <Editor example="Def‼ ←^ $\"_\\n_\" ⊃(/$\"_ ← _\"|/$\"Also_ ← _\")\nDef‼(X|5)\n+ X AlsoX"/>
        <p>"This is a simple example, but this concept can be used to create very powerful meta-programming tools."</p>

        <Hd id="compile-time-vs-run-time">"Compile Time vs Run Time"</Hd>
        <p>"The body of an array macro is always evaluated at compile time. One consequence of this is that bindings whose values cannot be known at compile time cannot be used in an array macro."</p>
        <p>"For example, because the value "<code>"5"</code>" is always the same, it is always known at compile time, and we can use a name that binds "<code>"5"</code>" in an array macro."</p>
        <Editor example="x ← 5\nF! ←^ $\"_ _\":x ⊢\nF!¯"/>
        <p>"However, if we use a value that cannot be known at compile time, like the result of the "<Prim prim=Rand/>" function, we will get an error."</p>
        <Editor example="x ← ⚂\nF! ←^ $\"_ _\":x ⊢\nF!¯"/> // Should fail
        <p>"There are two ways to work around this. The first is to simply put the code that generates the value in the macro itself."</p>
        <Editor example="F! ←^ $\"_ ⚂\" ⊢\nF!¯"/>
        <p>"The second is to use the "<Prim prim=Comptime/>" modifier, which forces its function to be evaluated at compile time."</p>
        <Editor example="x ← comptime(⚂)\nF! ←^ $\"_ _\":x ⊢\nF!¯"/>

        <Hd id="which-to-use">"What kind of macro should I use?"</Hd>
        <p>"Which kind of macro you use depends on what kind of code you are writing."</p>
        <p>"Array macros are much more powerful than stack macros, but they can be more complicated to write."</p>
        <p>"Additionally, stack macros are "<a href="https://en.wikipedia.org/wiki/hygienic_macro">"hygienic"</a>". When a stack macro refers to names of things, bindings you have definined in the surrounding code will not interfere; you will never accidentally use the wrong binding. Array macros make no such guarantees."</p>
        <p>"If you conceptually just want to define your own modifier, a stack macro is probably the simplest way to go."</p>
        <p>"If you want the full power (and all the complexity) of compile-time meta-programming, you'll need to use an array macro."</p>

        <Hd id="challenges">"Challenges"</Hd>

        <Challenge
            number=1
            prompt="creates a macro called F! which calls its function on each row of an array, reverses each row, and reverses the whole array"
            example="F!(⊂.) ↯3_4⇡12"
            answer="F! ← ⇌≡(⇌^!)"
            default="F! ← ^!"
            flip=true
            tests={&["F!(↯3) [1_2_3 4_5_6]", "F!(⊟.) 1_2 3_4"]}
            hidden="5"/>

        <Challenge
            number=2
            prompt="creates a macro called F‼ which calls its first function, then its second, then its first again."
            example="F‼⇌(⊂10) [1 2 3]"
            answer="F‼ ← ^!^!^!^,"
            default="F‼ ← ^!^!"
            flip=true
            tests={&["F‼⇌⍉ [1_2 3_4]", "F‼⊂⇌ 1_2 3_4 5_6"]}
            hidden="5"/>
    }
}

#[component]
fn TutorialTacitCode() -> impl IntoView {
    view! {
        { title_markdown("Tacit Code", "/text/tacit_code.md", ()).into_view() }

        <br/>
        <br/>
        <hr/>
        <p>"This is the end of the tutorial that pertains to writing "<em>"programs"</em>"."</p>
        <p>"If you want to use Uiua to write "<em>"software"</em>", then read on for the sections on modules and testing."</p>
        <p>"If you don't care about that stuff and want to learn more about the language, you can check out:"</p>
        <EndOfTutorialList/>
    }
}

#[component]
fn EndOfTutorialList() -> impl IntoView {
    view! {
        <ul>
            <li><A href="/docs#functions">"The list of all functions"</A></li>
            <li><A href="/docs#other-tutorials">"Other tutorials about more specific topics"</A></li>
            <li><A href="/docs#other-docs">"Other language topics"</A></li>
            <li>"The online "<A href="/pad">"pad"</A>" for writing longer code"</li>
        </ul>
    }
}

#[component]
fn TutorialModules() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Modules - Uiua Docs"/>
        <h1>"Modules"</h1>
        <p>"Modules are a way to organize your code in Uiua. They can either be defined in a scope, or imported from a file. Any Uiua file can be used as a module."</p>
        <p>"Modules can be compared to namespaces in other languages."</p>

        <Hd id="scoped-modules">"Scoped Modules"</Hd>
        <p>"Scoped modules are defined between a pair of "<code>"---"</code>"s. The first "<code>"---"</code>" should be immediately followed by a name for the module. Module names follow the same rules as other bindings."</p>
        <p>"Names from inside the module can be referenced by following the module name with a "<code>"~"</code>"."</p>
        <Editor example="---Mod\n  A ← 5\n  F ← +1\n  G ← F F\n---\nMod~G Mod~A"/>
        <p>"Bindings defined inside a scoped module are only visible inside the module."</p>
        <Editor example="---Mod\n  A ← 5\n  F ← +1\n  G ← F F\n---\nG A"/> // Should fail
        <p>"Names from inside the module can be "<em>"made"</em>" visible by following the module name with a "<code>"~"</code>" and a list of the names to make visible."</p>
        <Editor example="---Mod ~ A G\n  A ← 5\n  F ← +1\n  G ← F F\n---\nG A"/>
        <p>"Names defined above the module can be referenced inside it."</p>
        <Editor example="B ← 5\n---Mod\n  C ← ×2 B\n---\nMod~C"/>

        <Hd id="modules-as-functions">"Modules as Functions"</Hd>
        <p>"If a module defines a function called "<code>"Call"</code>" or "<code>"New"</code>", it can be called as a function. This can be useful if your module defines functions that all work on the same kind of data, like methods in some other languages."</p>
        <Editor example="---Foo\n  New ← {⊓$Bar$Baz}\n  Format ← /$\"_ _\"\n---\nFoo \"Hi!\" 5\nFoo~Format ."/>

        <Hd id="module-import-macros">"Module Import Macros"</Hd>
        <p>"If the name of a module is referenced as a macro (with a trailing "<code>"!"</code>"), names defined in the module will be available in the macro's scope."</p>
        <p>"This is useful if you need to refer to a bunch of bindings from a module without having to prefix them with the module name."</p>
        <Editor example="---Foo\n  New ← {⊓$Bar$Baz}\n  Format ← /$\"_ _\"\n  Incr ← ⍜(⊢⇌|+1)\n---\nFoo!(Format Incr New) \"Oh\" 10"/>

        <Hd id="web-files">"Files on the Website"</Hd>
        <p>"Using files as modules involves loading files from the file system."</p>
        <p>"This website has a virtual file system. You can write to virtual files with "<Prim prim=Sys(SysOp::FWriteAll)/>". You can also drag and drop files from your computer into the editor to make them available to import."</p>
        <p>"There is also a test module that can always be imported as "<code>"example.ua"</code>". Its contents is:"</p>
        <Editor example=EXAMPLE_UA/>

        <Hd id="importing">"Importing Modules"</Hd>
        <p>"Modules can be imported by file path with "<code>"~"</code>"."</p>
        <Editor example="~ \"example.ua\""/>
        <p>"This is not very useful on its own. We can bind items from the module in the current scope by listing them after the file path, separated by an additional "<code>"~"</code>"."</p>
        <Editor example="~ \"example.ua\" ~ Increment Square\n\nIncrement Square 3"/>
        <p>"If we have a lot of items to import, we can use multiple lines."</p>
        <Editor example="~ \"example.ua\"\n~ Increment Square\n~ Span\n~ Foo Bar\n\nIncrement Square Foo\nSpan 4 10"/>
        <p>"The formatter will automatically indent the imports if they are on multiple lines. It will also alphabetize them. Try it out!"</p>

        <Hd id="binding">"Binding Modules"</Hd>
        <p>"If we put a name before the import, we can bind the module to that name."</p>
        <p>"We can then reference items from that module anywhere using a "<code>"~"</code>"."</p>
        <Editor example="Ex ~ \"example.ua\"\n\nEx~Increment 10"/>
        <p>"This can be mixed and matched with the other import syntax."</p>
        <Editor example="Ex ~ \"example.ua\" ~ Increment Square\n\nEx~Double Square 3\nEx~Mac!×\nIncrement Ex~Bar"/>

        <Hd id="aliasing">"Aliasing Modules"</Hd>
        <p>"If you want to be able to refer to an item from a module with a different name, simply make a binding with the new name."</p>
        <Editor example="Ex ~ \"example.ua\"\nSqr ← Ex~Square\nSp ← Ex~Span\n\nSp⟜Sqr 3"/>
        <p>"These bindings will also get indented by the formatter if they immediately follow the import. Try formatting the above code!"</p>
        <p>"You can also re-bind the module itself."</p>
        <Editor example="Ex ~ \"example.ua\"\nLocalEx ← Ex\nLocalEx~Square 7"/>

        <Hd id="visibility">"Visibility"</Hd>
        <p>"All bindings in a module bound with the normal "<code>"←"</code>" arrow are public and can be used by importers of the module."</p>
        <p>"However, modules imported in modules, as well as their same-name imports (the names on lines that start with "<code>"~"</code>"), are private."</p>
        <p>"You may have noticed in the example file that one binding uses a special "<code>"↚"</code>" arrow. This indicates that the binding is private."</p>
        <p>"Private bindings cannot be accessed from outside the file in which they are defined."</p>
        <Editor example="~ \"example.ua\" ~ RangeDiff"/> // Should fail
        <p>"To enter this arrow, you can put a "<code>"~"</code>" after a binding's normal "<code>"←"</code>" or "<code>"="</code>"."</p>
        <p>"Try formatting the following example to see how this works."</p>
        <Editor example="A = +1\nB ← +2\nC =~ +3\nD ←~ +4"/>

        <Hd id="git-modules">"Git Modules"</Hd>
        <p>"Modules can be imported from Git repositories. Instead of a path, use a URL prefixed with "<code>"git:"</code>"."</p>
        <p>"The Uiua GitHub organization hosts an example module at "<a href="https://github.com/uiua-lang/example-module">"https://github.com/uiua-lang/example-module"</a>". The protocol specification can be omitted."</p>
        <Editor example="~ \"git: github.com/uiua-lang/example-module\" ~ Upscale\nUpscale 3 [1_2 3_4]"/>
        <p>"On the site, code is pulled from a "<code>"lib.ua"</code>" file at the root of the repository. Loading other files on the site is not supported."</p>
        <p>"To use Git modules in the "<A href="/docs/install">"native interpreter"</A>", you must have Git installed. The repository is cloned and the "<code>"lib.ua"</code>" file is loaded as the module's contents. Code from other files can be made available by importing them as modules in the "<code>"lib.ua"</code>" file."</p>
        <p>"The native interpreter also supports adding an additional "<code>"branch: <branch-name>"</code>" or "<code>"commit: <commit-hash>"</code>" specifier after the URL."</p>
        <p>"The "<code>"uiua module"</code>" command can be used to list or update Git modules."</p>
        <p>"You can find a curated list of Uiua modules "<a href="https://github.com/uiua-lang/uiua-modules">"here"</a>"."</p>
    }
}

#[component]
fn TutorialTesting() -> impl IntoView {
    use Primitive::*;
    view! {
        <Title text="Testing - Uiua Docs"/>
        <h1>"Testing"</h1>
        <Hd id="test-scopes">"Test Scopes"</Hd>
        <p>"A "<A href="/tutorial/modules#scoped-modules">"scoped module"</A>" with the name "<code>"test"</code>" is special in that is only run when the code is run with "<code>"uiua test"</code>". It is meant to be used with "<Prim prim=Assert/>"."</p>
        <Editor example="Square ← ×.\n---test\n⍤.=9 Square 3\n⍤.=225 Square 15\n---"/>
        <p><Prim prim=Assert/>" will return an error when its second argument is anything other than "<code>"1"</code>"."</p>
        <Editor example="Square ← ×.\n---test\n⍤.=25 Square 4\n---"/> // Should fail
        <p>"The first argument to "<Prim prim=Assert/>" is the value that will be thrown if the assertion fails. In the examples above, we have simply been "<Prim prim=Dup/>"ing the test value. We can throw a message instead."</p>
        <Editor example=r#"Square ← ×.
---test
⍤"3² is not 9!" =9 Square 3
⍤"4² is not 25!" =25 Square 4
---"#/>
        <p>"One nice pattern for writing tests is to put the expected result before the test computation and use "<Prims prims=[Assert, On, Match, Flip]/>"."</p>
        <p>"If the result does not match the expectation, that incorrect result will be thrown."</p>
        <Editor example="---test\n⍤⟜≍: 4 +2 2 # Passes\n---"/>
        <Editor example="---test\n⍤⟜≍: [2 3 5] +1 [1 2 3]\n---\n#     ↓↓↓↓↓↓"/> // Should fail

        <Hd id="run-modes">"Run Modes"</Hd>
        <p>"Whether tests will run or not depends on how you run the code."</p>
        <p>"On this website, both test and non-test code will always be run."</p>
        <p>"However, if you use the "<A href="/docs/install">"native interpreter"</A>", you have a few options."</p>
        <p><code>"uiua watch"</code>" will run all code, including tests."</p>
        <p><code>"uiua run"</code>" will only run non-test code."</p>
        <p><code>"uiua test"</code>" will only run test code, but also any non-test bindings and any non-test code which makes imports."</p>

        <br/>
        <br/>
        <hr/>
        <p>"Hooray! You've reached the end of the tutorial!"</p>
        <p>"To keep going with Uiua, you can check out:"</p>
        <EndOfTutorialList/>
    }
}
