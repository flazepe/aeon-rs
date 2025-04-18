use std::{collections::HashMap, sync::LazyLock};

pub static TIO_PROGRAMMING_LANGUAGES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("4", "4"),
        ("7", "7"),
        ("33", "33"),
        ("99", "99"),
        ("05ab1e", "05AB1E (legacy)"),
        ("1l-a", "1L_a"),
        ("1l-aoi", "1L_AOI"),
        ("2dfuck", "2DFuck"),
        ("2l", "2L"),
        ("2sable", "2sable"),
        ("3var", "3var"),
        ("a-gram", "a-gram"),
        ("a-pear-tree", "A Pear Tree"),
        ("abc", "ABC"),
        ("abc-assembler", "ABC-assembler"),
        ("accbb", "Acc!!"),
        ("aceto", "Aceto"),
        ("actually", "Actually"),
        ("ada-gnat", "Ada (GNAT)"),
        ("adapt", "Adapt"),
        ("addpp", "Add++"),
        ("adjust", "ADJUST"),
        ("agda", "Agda"),
        ("agony", "Agony"),
        ("ahead", "Ahead"),
        ("aheui", "Aheui (esotope)"),
        ("alchemist", "Alchemist"),
        ("algol68g", "ALGOL 68 (Genie)"),
        ("alice", "Alice"),
        ("alice-bob", "Alice & Bob"),
        ("aliceml", "Alice ML"),
        ("alphabeta", "AlphaBeta"),
        ("alphabetti-spaghetti", "Alphabetti spaghetti"),
        ("alphuck", "Alphuck"),
        ("alumin", "Alumin"),
        ("amnesiac-from-minsk", "The Amnesiac From Minsk"),
        ("ante", "Ante"),
        ("anyfix", "anyfix"),
        ("apl-dyalog", "APL (Dyalog Unicode)"),
        ("apl-dyalog-classic", "APL (Dyalog Classic)"),
        ("apl-dyalog-extended", "APL (Dyalog Extended)"),
        ("apl-dzaima", "APL (dzaima/APL)"),
        ("apl-ngn", "APL (ngn/apl)"),
        ("appleseed", "Appleseed"),
        ("arble", "ARBLE"),
        ("archway", "Archway"),
        ("archway2", "Archway2"),
        ("arcyou", "Arcyóu"),
        ("arnoldc", "ArnoldC"),
        ("asciidots", "AsciiDots"),
        ("asperix", "ASPeRiX"),
        ("assembly-as", "Assembly (as, x64, Linux)"),
        ("assembly-fasm", "Assembly (fasm, x64, Linux)"),
        ("assembly-gcc", "Assembly (gcc, x64, Linux)"),
        ("assembly-jwasm", "Assembly (JWasm, x64, Linux)"),
        ("assembly-nasm", "Assembly (nasm, x64, Linux)"),
        ("ats2", "ATS2"),
        ("attache", "Attache"),
        ("aubergine", "Aubergine"),
        ("awk", "AWK"),
        ("axo", "axo"),
        ("backhand", "Backhand"),
        ("bash", "Bash"),
        ("bc", "bc"),
        ("bctbww", "Bitwise Cyclic Tag But Way Worse"),
        ("bctbww2", "Bitwise Cyclic Tag But Way Worse 2.0"),
        ("beam", "Beam"),
        ("bean", "Bean"),
        ("beanshell", "BeanShell"),
        ("beatnik", "Beatnik"),
        ("beeswax", "Beeswax"),
        ("befunge", "Befunge-93"),
        ("befunge-93-fbbi", "Befunge-93 (FBBI)"),
        ("befunge-93-mtfi", "Befunge-93 (MTFI)"),
        ("befunge-93-pyfunge", "Befunge-93 (PyFunge)"),
        ("befunge-96-mtfi", "Befunge-96 (MTFI)"),
        ("befunge-97-mtfi", "Befunge-97 (MTFI)"),
        ("befunge-98", "Befunge-98 (FBBI)"),
        ("befunge-98-pyfunge", "Befunge-98 (PyFunge)"),
        ("bit", "Bit"),
        ("bitbitjump", "BitBitJump"),
        ("bitch", "bitch"),
        ("bitch-bith", "bitch (bit-h)"),
        ("bitch-shifty", "bitch (shifty)"),
        ("bitchanger", "BitChanger"),
        ("bitcycle", "BitCycle"),
        ("bitwise", "Bitwise"),
        ("blak", "Black (blak)"),
        ("blc", "Binary Lambda Calculus"),
        ("boo", "Boo"),
        ("boolfuck", "Boolfuck"),
        ("bosh", "bosh"),
        ("bot-engine", "Bot Engine"),
        ("brachylog", "Brachylog v1"),
        ("brachylog2", "Brachylog"),
        ("bracmat", "Bracmat"),
        ("braille", "Braille"),
        ("brain-flak", "Brain-Flak"),
        ("brainbash", "Brainbash"),
        ("brainbool", "brainbool"),
        ("brainflump", "BrainFlump"),
        ("brainfuck", "brainfuck"),
        ("braingolf", "Braingolf"),
        ("brainhack", "Brain-Flak (BrainHack)"),
        ("brainlove", "Brainlove"),
        ("brainspace", "BrainSpace"),
        ("brat", "Brat"),
        ("brian-chuck", "Brian & Chuck"),
        ("broccoli", "Broccoli"),
        ("bubblegum", "Bubblegum"),
        ("burlesque", "Burlesque"),
        ("buzzfizz", "BuzzFizz"),
        ("bwfuckery", "Bitwise Fuckery"),
        ("c-clang", "C (clang)"),
        ("c-gcc", "C (gcc)"),
        ("c-tcc", "C (tcc)"),
        ("caboose", "Caboose"),
        ("cakeml", "CakeML"),
        ("calc2", "calc (TTK)"),
        ("canvas", "Canvas"),
        ("cardinal", "Cardinal"),
        ("carol-dave", "Carol & Dave"),
        ("carrot", "Carrot"),
        ("cascade", "Cascade"),
        ("catholicon", "Catholicon"),
        ("cauliflower", "Cauliflower"),
        ("ceres", "Ceres"),
        ("ceylon", "Ceylon"),
        ("chain", "Chain"),
        ("charm", "Charm"),
        ("chef", "Chef"),
        ("changeling", "Changeling"),
        ("chapel", "Chapel"),
        ("charcoal", "Charcoal"),
        ("check", "Check"),
        ("checkedc", "Checked C"),
        ("cheddar", "Cheddar"),
        ("chip", "Chip"),
        ("cil-mono", "CIL (Mono IL assembler)"),
        ("cinnamon-gum", "Cinnamon Gum"),
        ("cixl", "cixl"),
        ("cjam", "CJam"),
        ("clam", "Clam"),
        ("clean", "Clean"),
        ("clips", "CLIPS"),
        ("clisp", "Common Lisp"),
        ("clojure", "Clojure"),
        ("cobol-gnu", "COBOL (GNU)"),
        ("cobra", "Cobra"),
        ("coconut", "Coconut"),
        ("coffeescript", "CoffeeScript 1"),
        ("coffeescript2", "CoffeeScript 2"),
        ("commata", ",,,"),
        ("commentator", "Commentator"),
        ("commercial", "Commercial"),
        ("condit", "Condit"),
        ("convex", "Convex"),
        ("cood", "Cood"),
        ("corea", "Corea"),
        ("cow", "COW"),
        ("cpp-clang", "C++ (clang)"),
        ("cpp-gcc", "C++ (gcc)"),
        ("cpy", "CPY"),
        ("cquents", "cQuents"),
        ("crayon", "Crayon"),
        ("cryptol", "Cryptol"),
        ("crystal", "Crystal"),
        ("cs-core", "C# (.NET Core)"),
        ("cs-csc", "C# (Visual C# Compiler)"),
        ("cs-csi", "C# (Visual C# Interactive Compiler)"),
        ("cs-mono", "C# (Mono C# compiler)"),
        ("cs-mono-shell", "C# (Mono C# Shell)"),
        ("csl", "CSL"),
        ("cubically", "Cubically"),
        ("cubix", "Cubix"),
        ("curlyfrick", "Curlyfrick"),
        ("curry-pakcs", "Curry (PAKCS)"),
        ("curry-sloth", "Curry (Sloth)"),
        ("cy", "Cy"),
        ("cyclone", "Cyclone"),
        ("d", "D"),
        ("d2", "D2"),
        ("dafny", "Dafny"),
        ("dart", "Dart"),
        ("dash", "Dash"),
        ("dc", "dc"),
        ("deadfish-", "Deadfish~"),
        ("decimal", "Decimal"),
        ("delimit", "Del|m|t"),
        ("deorst", "Deorst"),
        ("dirty", "Dirty"),
        ("detour", "Detour"),
        ("dg", "dg"),
        ("dobela", "DOBELA"),
        ("dobela-dobcon", "DOBELA (dobcon)"),
        ("dodos", "Dodos"),
        ("dreaderef", "Dreaderef"),
        ("drive-in-window", "Drive-In Window"),
        ("dscript", "DScript"),
        ("dstack", "DStack"),
        ("eacal", "eacal"),
        ("ec", "eC"),
        ("ecndpcaalrlp", "!@#$%^&*()_+"),
        ("ecpp-c", "ecpp + C (gcc)"),
        ("ecpp-cpp", "ecpp + C++ (gcc)"),
        ("dyvil", "Dyvil"),
        ("ed", "ed"),
        ("egel", "Egel"),
        ("element", "Element"),
        ("elf", "ELF (x86/x64, Linux)"),
        ("elixir", "Elixir"),
        ("elvm-ir", "ELVM-IR"),
        ("emacs-lisp", "Emacs Lisp"),
        ("emmental", "Emmental"),
        ("emoji", "Emoji"),
        ("emoji-gramming", "Emoji-gramming"),
        ("emojicode", "Emojicode 0.5"),
        ("emojicode6", "Emojicode"),
        ("emojicoder", "EmojiCoder"),
        ("emotifuck", "emotifuck"),
        ("emotinomicon", "Emotinomicon"),
        ("empty-nest", "(())"),
        ("enlist", "Enlist"),
        ("erlang-escript", "Erlang (escript)"),
        ("es", "es"),
        ("esopunk", "ESOPUNK"),
        ("eta", "ETA"),
        ("euphoria3", "Euphoria 3"),
        ("euphoria4", "Euphoria 4"),
        ("evil", "evil"),
        ("explode", "Explode"),
        ("extended-brainfuck-type-i", "Extended Brainfuck Type I"),
        ("extrac", "ExtraC"),
        ("face", "face"),
        ("factor", "Factor"),
        ("false", "FALSE"),
        ("fantom", "Fantom"),
        ("farnsworth", "Farnsworth"),
        ("felix", "Felix"),
        ("fernando", "FerNANDo"),
        ("feu", "FEU"),
        ("fimpp", "FIM++"),
        ("fish", "><>"),
        ("fish-shell", "fish"),
        ("fission", "Fission"),
        ("fission2", "Fission 2"),
        ("flipbit", "Flipbit"),
        ("floater", "Floater"),
        ("flobnar", "Flobnar"),
        ("foam", "Foam"),
        ("focal", "FOCAL-69"),
        ("foo", "Foo"),
        ("forget", "Forget"),
        ("forked", "Forked"),
        ("forte", "Forte"),
        ("forth-gforth", "Forth (gforth)"),
        ("fortran-gfortran", "Fortran (GFortran)"),
        ("fourier", "Fourier"),
        ("fractran", "FRACTRAN"),
        ("fs-core", "F# (.NET Core)"),
        ("fs-mono", "F# (Mono)"),
        ("fueue", "Fueue"),
        ("funciton", "Funciton"),
        ("functoid", "Functoid"),
        ("funky", "Funky"),
        ("funky2", "Funky 2"),
        ("fynyl", "Fynyl"),
        ("gaia", "Gaia"),
        ("gaotpp", "Gaot++"),
        ("gap", "GAP"),
        ("gema", "Gema"),
        ("geo", "Geo"),
        ("glypho", "Glypho"),
        ("glypho-shorthand", "Glypho (shorthand)"),
        ("gnuplot", "gnuplot"),
        ("go", "Go"),
        ("golfish", "Gol><>"),
        ("golfscript", "GolfScript"),
        ("granule", "Granule"),
        ("grass", "Grass"),
        ("grime", "Grime"),
        ("groovy", "Groovy"),
        ("gs2", "GS2"),
        ("gwion", "Gwion"),
        ("hades", "HadesLang"),
        ("haskell", "Haskell"),
        ("haskell-gofer", "Haskell 1.2 (Gofer)"),
        ("haskell-hugs", "Haskell 98 (Hugs)"),
        ("haskell-literate", "Literate Haskell"),
        ("hasm", "hASM"),
        ("haxe", "Haxe"),
        ("haystack", "Haystack"),
        ("hbcht", "Half-Broken Car in Heavy Traffic"),
        ("hdbf", "Hyper-Dimensional Brainfuck"),
        ("hexagony", "Hexagony"),
        ("hobbes", "Hobbes"),
        ("hodor", "Hodor"),
        ("homespring", "Homespring"),
        ("hspal", "Hexadecimal Stacking Pseudo-Assembly Language"),
        ("huginn", "Huginn"),
        ("husk", "Husk"),
        ("hy", "Hy"),
        ("i", "I"),
        ("iag", "iag"),
        ("icon", "Icon"),
        ("idris", "Idris"),
        ("incident", "Incident"),
        ("ink", "ink"),
        ("intercal", "INTERCAL"),
        ("io", "Io"),
        ("j", "J"),
        ("jael", "JAEL"),
        ("jq", "jq"),
        ("jx", "Jx"),
        ("j-uby", "J-uby"),
        ("japt", "Japt"),
        ("java-jdk", "Java (JDK)"),
        ("java-openjdk", "Java (OpenJDK 8)"),
        ("javascript-babel-node", "JavaScript (Babel Node)"),
        ("javascript-node", "JavaScript (Node.js)"),
        ("javascript-spidermonkey", "JavaScript (SpiderMonkey)"),
        ("javascript-v8", "JavaScript (V8)"),
        ("jelly", "Jelly"),
        ("jellyfish", "Jellyfish"),
        ("joy", "Joy"),
        ("julia", "Julia 0.4"),
        ("julia1x", "Julia 1.0"),
        ("julia5", "Julia 0.5"),
        ("julia6", "Julia 0.6"),
        ("julia7", "Julia 0.7"),
        ("k-kona", "K (Kona)"),
        ("k-ngn", "K (ngn/k)"),
        ("k-ok", "K (oK)"),
        ("kavod", "kavod"),
        ("keg", "Keg"),
        ("kipple-cipple", "Kipple (cipple)"),
        ("klein", "Klein"),
        ("koberi-c", "Kobeři-C"),
        ("koka", "Koka"),
        ("kotlin", "Kotlin"),
        ("krrp", "krrp"),
        ("ksh", "ksh"),
        ("l33t", "l33t"),
        ("labyrinth", "Labyrinth"),
        ("lean", "Lean"),
        ("lily", "Lily"),
        ("llvm", "LLVM IR"),
        ("lmbm", "Lean Mean Bean Machine"),
        ("lnusp", "LNUSP"),
        ("locksmith", "Locksmith"),
        ("logicode", "Logicode"),
        ("lolcode", "LOLCODE"),
        ("lost", "Lost"),
        ("lower", "LOWER"),
        ("lua", "Lua"),
        ("lua-luajit", "Lua (LuaJIT)"),
        ("lua-openresty", "Lua (OpenResty)"),
        ("ly", "Ly"),
        ("m", "M"),
        ("m4", "M4"),
        ("machinecode", "MachineCode"),
        ("make", "Make"),
        ("malbolge", "Malbolge"),
        ("malbolge-unshackled", "Malbolge Unshackled"),
        ("mamba", "Mamba"),
        ("mariolang", "MarioLANG"),
        ("mascarpone", "Mascarpone"),
        ("mathgolf", "MathGolf"),
        ("mathematica", "Wolfram Language (Mathematica)"),
        ("mathics", "Mathics"),
        ("matl", "MATL"),
        ("maverick", "Maverick"),
        ("maxima", "Maxima"),
        ("maybelater", "MaybeLater"),
        ("memory-gap", "Memory GAP"),
        ("milambda", "MiLambda"),
        ("milky-way", "Milky Way"),
        ("minefriff", "MineFriff"),
        ("minimal-2d", "Minimal-2D"),
        ("miniml", "miniML"),
        ("minkolang", "Minkolang"),
        ("mirror", "Mirror"),
        ("momema", "Momema"),
        ("monkeys", "Monkeys"),
        ("moonscript", "Moonscript"),
        ("moorhens", "Moorhens"),
        ("mornington-crescent", "Mornington Crescent"),
        ("mouse", "Mouse-79"),
        ("mouse2002", "Mouse-2002"),
        ("mouse83", "Mouse-83"),
        ("mu6", "µ6"),
        ("mumps", "MUMPS"),
        ("muriel", "Muriel"),
        ("my", "MY"),
        ("my-basic", "MY-BASIC"),
        ("nameless", "nameless language"),
        ("nandy", "Nandy"),
        ("nial", "Nial"),
        ("nikud", "Nikud"),
        ("nim", "Nim"),
        ("neim", "Neim"),
        ("neutrino", "Neutrino"),
        ("nhohnhehr", "Nhohnhehr"),
        ("no", "No"),
        ("noether", "Noether"),
        ("nqt", "NotQuiteThere"),
        ("ntfjc", "NTFJ (NTFJC)"),
        ("numberwang", "Numberwang"),
        ("oasis", "Oasis"),
        ("obcode", "ObCode"),
        ("oberon-07", "Oberon-07"),
        ("object-pascal-fpc", "Object Pascal (FPC)"),
        ("objective-c-clang", "Objective-C (clang)"),
        ("objective-c-gcc", "Objective-C (gcc)"),
        ("occam-pi", "occam-pi"),
        ("ocaml", "OCaml"),
        ("octave", "Octave"),
        ("odin", "Odin"),
        ("ohm", "Ohm"),
        ("ohm2", "Ohm v2"),
        ("oml", "OML"),
        ("ooocode", "oOo CODE"),
        ("oration", "Oration"),
        ("ork", "ORK"),
        ("orst", "Orst"),
        ("osabie", "05AB1E"),
        ("osh", "OSH"),
        ("pain-flak", "Pain-Flak"),
        ("paradoc", "Paradoc"),
        ("parenthesis-hell", "Parenthesis Hell"),
        ("parenthetic", "Parenthetic"),
        ("pari-gp", "Pari/GP"),
        ("pascal-fpc", "Pascal (FPC)"),
        ("path", "PATH"),
        ("pbrain", "pbrain"),
        ("perl4", "Perl 4"),
        ("perl5", "Perl 5"),
        ("perl5-cperl", "Perl 5 (cperl)"),
        ("perl6", "Perl 6"),
        ("perl6-niecza", "Perl 6 (Niecza)"),
        ("phoenix", "Phoenix"),
        ("phooey", "Phooey"),
        ("php", "PHP"),
        ("physica", "Physica"),
        ("picolisp", "PicoLisp"),
        ("piet", "Piet"),
        ("pike", "Pike"),
        ("pilot-pspilot", "PILOT (psPILOT)"),
        ("pilot-rpilot", "PILOT (RPilot)"),
        ("pingpong", "PingPong"),
        ("pip", "Pip"),
        ("pixiedust", "Pixiedust"),
        ("pl", "pl"),
        ("pony", "Pony"),
        ("positron", "Positron"),
        ("postl", "PostL"),
        ("postscript-xpost", "PostScript (xpost)"),
        ("powershell", "PowerShell"),
        ("powershell-core", "PowerShell Core"),
        ("prelude", "Prelude"),
        ("premier", "Premier"),
        ("preproc", "Preproc"),
        ("prolog-ciao", "Prolog (Ciao)"),
        ("prolog-swi", "Prolog (SWI)"),
        ("proton", "Proton"),
        ("proton2", "Proton 2.0"),
        ("ps-core", "P#"),
        ("pure", "Pure"),
        ("purescript", "PureScript"),
        ("purple", "Purple"),
        ("pushy", "Pushy"),
        ("puzzlang", "Puzzlang"),
        ("pyke", "Pyke"),
        ("pylons", "Pylons"),
        ("pyn-tree", "PynTree"),
        ("pyon", "Pyon"),
        ("pyramid-scheme", "Pyramid Scheme"),
        ("pyret", "Pyret"),
        ("pyt", "Pyt"),
        ("pyth", "Pyth"),
        ("python1", "Python 1"),
        ("python2", "Python 2"),
        ("python2-cython", "Python 2 (Cython)"),
        ("python2-iron", "Python 2 (IronPython)"),
        ("python2-jython", "Python 2 (Jython)"),
        ("python2-pypy", "Python 2 (PyPy)"),
        ("python3", "Python 3"),
        ("python38pr", "Python 3.8 (pre-release)"),
        ("python3-cython", "Python 3 (Cython)"),
        ("python3-pypy", "Python 3 (PyPy)"),
        ("python3-stackless", "Python 3 (Stackless)"),
        ("qqq", "???"),
        ("qs-core", "Q#"),
        ("quadr", "QuadR"),
        ("quadrefunge-97-mtfi", "Quadrefunge-97 (MTFI)"),
        ("quads", "QuadS"),
        ("quarterstaff", "Quarterstaff"),
        ("quintefunge-97-mtfi", "Quintefunge-97 (MTFI)"),
        ("r", "R"),
        ("racket", "Racket"),
        ("rad", "RAD"),
        ("rail", "Rail"),
        ("random-brainfuck", "Random Brainfuck"),
        ("rapira", "Rapira"),
        ("re-direction", "Re:direction"),
        ("reason", "Reason"),
        ("rebol", "REBOL"),
        ("rebol3", "REBOL 3"),
        ("recursiva", "Recursiva"),
        ("red", "Red"),
        ("reng", "Reng"),
        ("reregex", "ReRegex"),
        ("res", "res"),
        ("resplicate", "ResPlicate"),
        ("reticular", "Reticular"),
        ("retina", "Retina 0.8.2"),
        ("retina1", "Retina"),
        ("return", "RETURN"),
        ("rexx", "Rexx (Regina)"),
        ("ring", "Ring"),
        ("rk", "rk"),
        ("rockstar", "Rockstar"),
        ("roda", "Röda"),
        ("roop", "ROOP"),
        ("ropy", "Ropy"),
        ("rotor", "Rotor"),
        ("rprogn", "RProgN"),
        ("rprogn-2", "RProgN 2"),
        ("ruby", "Ruby"),
        ("runic", "Runic Enchantments"),
        ("rust", "Rust"),
        ("rutger", "Rutger"),
        ("sad-flak", "Sad-Flak"),
        ("sakura", "Sakura"),
        ("sbf", "Symbolic Brainfuck"),
        ("scala", "Scala"),
        ("scheme-chez", "Chez Scheme"),
        ("scheme-chicken", "CHICKEN Scheme"),
        ("scheme-gambit", "Gambit Scheme (gsi)"),
        ("scheme-guile", "Guile"),
        ("sed", "sed 4.2.2"),
        ("sed-gnu", "sed"),
        ("seed", "Seed"),
        ("septefunge-97-mtfi", "Septefunge-97 (MTFI)"),
        ("seriously", "Seriously"),
        ("sesos", "Sesos"),
        ("set", "Set"),
        ("sexefunge-97-mtfi", "Sexefunge-97 (MTFI)"),
        ("sfk", "sfk"),
        ("shapescript", "ShapeScript"),
        ("shnap", "Shnap"),
        ("shortc", "shortC"),
        ("shove", "Shove"),
        ("shp", ";#+"),
        ("shtriped", "Shtriped"),
        ("silos", "S.I.L.O.S"),
        ("sidef", "Sidef"),
        ("silberjoder", "Silberjoder"),
        ("simplefunge", "Simplefunge"),
        ("simplestack", "Implicit"),
        ("simplex", "Simplex"),
        ("simula", "Simula (cim)"),
        ("sisal", "SISAL"),
        ("sisi", "Sisi"),
        ("slashes", "///"),
        ("smbf", "Self-modifying Brainfuck"),
        ("sml-mlton", "Standard ML (MLton)"),
        ("smol", "smol"),
        ("snails", "Snails"),
        ("snobol4", "SNOBOL4 (CSNOBOL4)"),
        ("snowman", "Snowman"),
        ("snusp", "SNUSP (Modular)"),
        ("snusp-bloated", "SNUSP (Bloated)"),
        ("snuspi", "SNUSP (Snuspi)"),
        ("somme", "Somme"),
        ("spaced", "Spaced"),
        ("spim", "Assembly (MIPS, SPIM)"),
        ("spl", "Shakespeare Programming Language"),
        ("spoon", "Spoon"),
        ("sqlite", "SQLite"),
        ("squirrel", "Squirrel"),
        ("stackcats", "Stack Cats"),
        ("stacked", "Stacked"),
        ("starfish", "*><>"),
        ("starry", "Starry"),
        ("stax", "Stax"),
        ("stencil", "Stencil"),
        ("stones", "Stones"),
        ("str", "str"),
        ("straw", "Straw"),
        ("subskin", "Subskin"),
        ("sumerian", "Sumerian"),
        ("supermariolang", "SuperMarioLang"),
        ("superstack", "Super Stack!"),
        ("surface", "Surface"),
        ("swap", "Swap"),
        ("swift4", "Swift"),
        ("syms", "Syms"),
        ("symbolic-python", "Symbolic Python"),
        ("taco", "TacO"),
        ("tampio", "Tampio (functional)"),
        ("tampioi", "Tampio (imperative)"),
        ("tamsin", "Tamsin"),
        ("tapebagel", "TapeBagel"),
        ("taxi", "Taxi"),
        ("tcl", "Tcl"),
        ("tcsh", "tcsh"),
        ("templat", "TemplAt"),
        ("templates", "Templates Considered Harmful"),
        ("thing", "Thing"),
        ("threead", "Threead"),
        ("thue", "Thue"),
        ("thutu", "Thutu"),
        ("tidy", "Tidy"),
        ("tincan", "TinCan"),
        ("tinybf", "tinyBF"),
        ("tinylisp", "tinylisp"),
        ("tir", "Tir"),
        ("tis", "TIS"),
        ("toi", "Toi"),
        ("tmbww", "Turing Machine But Way Worse"),
        ("transcript", "TRANSCRIPT"),
        ("trefunge-97-mtfi", "Trefunge-97 (MTFI)"),
        ("trefunge-98-pyfunge", "Trefunge-98 (PyFunge)"),
        ("triangular", "Triangular"),
        ("triangularity", "Triangularity"),
        ("trigger", "Trigger"),
        ("triple-threat", "Triple Threat"),
        ("trumpscript", "TrumpScript"),
        ("turtled", "Turtlèd"),
        ("typescript", "TypeScript"),
        ("ubasic", "uBASIC"),
        ("underload", "Underload"),
        ("unefunge-97-mtfi", "Unefunge-97 (MTFI)"),
        ("unefunge-98-pyfunge", "Unefunge-98 (PyFunge)"),
        ("unicat", "Unicat"),
        ("unlambda", "Unlambda"),
        ("uno", "Uno"),
        ("unreadable", "Unreadable"),
        ("ursala", "Ursala"),
        ("v", "V (vim)"),
        ("v-fmota", "V (FMota)"),
        ("vala", "Vala"),
        ("var", "VAR"),
        ("vb-core", "Visual Basic .NET (.NET Core)"),
        ("verbosity", "Verbosity"),
        ("verbosity2", "Verbosity v2"),
        ("versert", "Versert"),
        ("visual-basic-net-mono", "Visual Basic .NET (Mono)"),
        ("visual-basic-net-vbc", "Visual Basic .NET (VBC)"),
        ("vitsy", "Vitsy"),
        ("vlang", "V (vlang.io)"),
        ("vsl", "VSL"),
        ("wasm", "WebAssembly (WaWrapper)"),
        ("waterfall", "The Waterfall Model"),
        ("whirl", "Whirl"),
        ("whispers", "Whispers v1"),
        ("whispers2", "Whispers v2"),
        ("whitespace", "Whitespace"),
        ("width", "Width"),
        ("wierd", "Wierd (John's)"),
        ("wise", "Wise"),
        ("woefully", "Woefully"),
        ("wren", "Wren"),
        ("wsf", "wsf"),
        ("wumpus", "Wumpus"),
        ("wyalhein", "W.Y.A.L.H.E.I.N."),
        ("xeec", "xEec"),
        ("xeraph", "xeraph"),
        ("yaball", "YABALL"),
        ("yabasic", "Yabasic"),
        ("yash", "yash"),
        ("ybc", "B (ybc)"),
        ("yup", "yup"),
        ("z3", "Z3"),
        ("z80golf", "Z80Golf"),
        ("zephyr", "Zephyr"),
        ("zig", "Zig"),
        ("zkl", "zkl"),
        ("zoidberg", "Zoidberg"),
        ("zsh", "Zsh"),
    ])
});
pub static TIO_PROGRAMMING_LANGUAGE_CODES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("assembly", "assembly-nasm"),
        ("asm", "assembly-nasm"),
        ("bf", "brainfuck"),
        ("c", "c-clang"),
        ("c++", "cpp-clang"),
        ("cpp", "cpp-clang"),
        ("cxx", "cpp-clang"),
        ("c#", "cs-core"),
        ("cs", "cs-core"),
        ("csharp", "cs-core"),
        ("coffee", "coffeescript"),
        ("ex", "elixir"),
        ("exs", "elixir"),
        ("f#", "fs-core"),
        ("fs", "fs-core"),
        ("fsharp", "fs-core"),
        ("hs", "haskell"),
        ("java", "java-jdk"),
        ("javascipt", "javascript-node"),
        ("js", "javascript-node"),
        ("kt", "kotlin"),
        ("pascal", "pascal-fpc"),
        ("perl", "perl5"),
        ("ps", "powershell"),
        ("ps1", "powershell"),
        ("py", "python3"),
        ("python", "python3"),
        ("rs", "rust"),
        ("rb", "ruby"),
        ("sh", "bash"),
        ("ts", "typescript"),
        ("vb", "vb-core"),
    ])
});
