
# Table of Contents

1.  [ZyBS](#orgcd99896)
    1.  [Design Goals](#org5ee4105)
    2.  [File Types](#orga5d19f4)
2.  [Module](#org425f280)
    1.  [Fields](#orgcaf0c45)
3.  [Screen](#org490d75f)
    1.  [Leaf Screens](#org64f04c2)
    2.  [Composite Screens](#orgafa202f)
    3.  [Params and Sigils](#org3619c85)
    4.  [Output](#orgd24ffe2)
        1.  [Buckets](#orgde1259e)
        2.  [Output Validation](#orgf1e79e0)
    5.  [Directory Dependencies](#org62b0148)
4.  [Interpolation Grammar](#org946fa96)
    1.  [Sigils](#orga2c5151)
    2.  [Escaping](#org23bf2d3)
    3.  [Formatting](#orgc428bbe)
5.  [Directory Discovery](#org3c2260f)
    1.  [Declaring Directories](#orgca7e6c3)
    2.  [The `$discover` Builtin](#org49ea377)
6.  [Execution Semantics](#org7093327)
    1.  [Output Assembly](#org9634628)
    2.  [Global Mutation](#orgc280c40)
7.  [`.zyl` File](#org524a0d0)
    1.  [Top-Matter](#orge58d847)
    2.  [Format](#orgec9fa2a)
    3.  [Screen Fields](#org1201310)
8.  [Zyfile](#orgefe5a8e)
    1.  [Grammar](#org0f01231)
    2.  [Naming](#orgcd025e0)
    3.  [Validation and Peeling](#org21341bb)
9.  [.zystate](#org8d99f8a)
    1.  [Fields](#org78e7a74)
    2.  [`zybs peel` and `zybs return`](#org2617583)
    3.  [Shared Screens (DAG) Caveat](#org9c3253d)
10. [`.zydoc` File](#org0ac67a9)
    1.  [Format](#orgb037b48)
    2.  [Top-Matter](#org3b09900)
    3.  [Screen Documentation Fields](#orgf934d85)
11. [Command Line Interface](#org840fbc6)
    1.  [Global Flags](#orgfa2dfec)
    2.  [Project Lifecycle](#org4f28fa6)
    3.  [Screen State](#orgbc98049)
    4.  [Inspection](#org1b8dd0d)
    5.  [Validation](#org882f0d0)
    6.  [Authoring](#org7ebf8b9)
    7.  [Debug Output](#org3f5d73d)
12. [Future Considerations](#org7b2eac1)



<a id="orgcd99896"></a>

# ZyBS

ZyBS is a build-system abstraction framework.
It presents a build system as a hierarchy of `Screens`, from which a project selects a single entry point.
Abstraction depth is chosen per project and adjusted incrementally.


<a id="org5ee4105"></a>

## Design Goals

-   A build collapses to a single entry `Screen`, invoked by one command.
-   Any `Screen` may be *peeled*, exposing the `Screens` or literal build-system code one level beneath it.
-   Each peeled `Screen` has one named responsibility, which the user assumes by peeling it; never more, never less.
-   Each `Screen` handles exactly one responsibility.

A composite `Screen's` responsibility is the one it introduces at its own level.

-   No tool writes to a `Zyfile`.

A still-failing build with a specific, screen-named error is the correction mechanism; see [8.3](#org21341bb).

-   `.zydoc` pedagogy is optional and purely additive; a build never depends on it.
-   The CLI and GUI are peer clients of one API; neither exposes an operation the other cannot.

See [11](#org840fbc6).


<a id="orga5d19f4"></a>

## File Types

ZyBS uses three file types, each separately authored:

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">File</th>
<th scope="col" class="org-left">Author</th>
<th scope="col" class="org-left">Role</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>.zyl</code></td>
<td class="org-left">experienced developer</td>
<td class="org-left">Defines the abstraction: the full <code>Screen</code> hierarchy for one build system. See <a href="#org524a0d0">7</a>.</td>
</tr>

<tr>
<td class="org-left"><code>Zyfile</code></td>
<td class="org-left">project user</td>
<td class="org-left">Per-project. Selects one entry <code>Screen</code> from a <code>.zyl</code> and supplies its arguments. Custom grammar, not YAML. See <a href="#orgefe5a8e">8</a>.</td>
</tr>

<tr>
<td class="org-left"><code>.zydoc</code></td>
<td class="org-left">experienced developer</td>
<td class="org-left">Pedagogical only: <code>Module</code> and per-<code>Screen</code> descriptions and responsibilities. No effect on execution. See <a href="#org0ac67a9">10</a>.</td>
</tr>
</tbody>
</table>

A build requires only a `.zyl` and a `Zyfile`; `.zydoc` is always optional.
When present, a `.zydoc` shares its `.zyl`&rsquo;s base name (eg. `CMake.zyl` and `CMake.zydoc`).


<a id="org425f280"></a>

# Module

A `Module` is an explicitly authored grouping of `Screens` at a similar conceptual level, used to structure the learning progression.

`Modules` are not a structural or derived property of the `Screen` hierarchy.
Structural depth (`Subscreen` nesting) is determined by the graph and needs no separate bookkeeping.
`Module` membership is an independent editorial judgment: a structurally deep `Screen` may be conceptually simple, and a structurally shallow one may not be.

`Modules` are ordered.
A learner peeling a `Screen` whose `Module` is significantly later than the `Modules` of their already-peeled `Screens` may be taking on something they are not ready for or do not yet need.
Tooling (CLI/GUI) surfaces a caution in that case.
This is a runtime/UX behavior, not a schema validation rule.
See [6](#org7093327).

`Modules` are defined in the `.zydoc` file, not the `.zyl` file; see [10](#org0ac67a9).


<a id="orgcaf0c45"></a>

## Fields

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Type</th>
<th scope="col" class="org-left">Required</th>
<th scope="col" class="org-left">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>id</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">Unique module identifier.</td>
</tr>

<tr>
<td class="org-left"><code>order</code></td>
<td class="org-left">integer</td>
<td class="org-left">yes</td>
<td class="org-left">Explicit ordinal, used to compute inter-module distance for readiness cautions. Not inferred from list position.</td>
</tr>

<tr>
<td class="org-left"><code>title</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">Short learner-facing name.</td>
</tr>

<tr>
<td class="org-left"><code>description</code></td>
<td class="org-left">string (block)</td>
<td class="org-left">no</td>
<td class="org-left">Prose describing what the module covers conceptually.</td>
</tr>

<tr>
<td class="org-left"><code>screens</code></td>
<td class="org-left">list of string</td>
<td class="org-left">yes</td>
<td class="org-left"><code>Screen</code> names belonging to this module.</td>
</tr>
</tbody>
</table>

Every `Screen` defined in the corresponding `.zyl` file must appear in exactly one `Module's` `screens` list: no orphans, no duplicate placement.


<a id="org490d75f"></a>

# Screen

A `Screen` abstracts either a single piece of literal build-system code or a group of less-abstracted `Screens`.
A `Screen` is exactly one of two mutually exclusive kinds:

-   A *leaf* screen: has `commands` (literal build-system code) and nothing else.
-   A *composite* screen: has `subscreens` (a call graph into other `Screens`) and nothing else.

A `Screen` has exactly one of `commands` or `subscreens`; never both, never neither.


<a id="org64f04c2"></a>

## Leaf Screens

A leaf screen holds `commands`: an ordered list of literal, pure build-system code.
`commands` must not reference other `Screens` by name.
They are opaque strings except for [interpolation](#org946fa96) purposes.

    target_sources:
      params: [sources]
      output: src/CMakeLists.txt
      commands:
        - add_library(core_sources OBJECT @sources)


<a id="orgafa202f"></a>

## Composite Screens

A composite screen holds `subscreens`: an ordered list of calls into other `Screens`.
Each call names the target `Screen` and binds arguments explicitly to that `Screen's` declared `params`.
Argument binding is entirely at the calling `Screen's` discretion; arguments are never filtered down implicitly from a parent&rsquo;s own `params` or `vars`.

    binary:
      params: [bin_name]
      subscreens:
        - screen: target_sources
          args:
            sources: ${build_dir}/main.cpp
      commands:
        - add_executable(@bin_name ...)

The same `Screen` may be called from multiple parents.
The call structure is a DAG, not a strict tree; each parent supplies its own argument bindings independently.


<a id="org3619c85"></a>

## Params and Sigils

A `Screen` declares its `params` up front.
Each param may optionally be *sigiled*:

-   An unsigiled param (eg. `sources`) must be supplied via `args` at every call site.
-   A `$`-sigiled param (eg. `$cxx_flags`) is automatically bound at invocation to the global var of the same name; syntactic sugar for an implicit `args` binding to that global.

Binding a `$`-sigiled param *writes* the supplied value into that global var, mutating it for the remainder of execution; see [6.2](#orgc280c40).
An explicit `vars:` entry at a call site may override this implicit binding, but must produce a linter warning, since it silently defeats the `Screen's` declared binding.

See [4](#org946fa96) for full sigil, escaping, and formatting rules used inside `commands` and `output`.


<a id="orgd24ffe2"></a>

## Output

A leaf `Screen` must declare `output`: the file its `commands` are written to.
It may optionally declare `bucket`: the named region of that file its content belongs to.
Composite `Screens` never declare `output` or `bucket`; they perform no emission of their own.
Output is strictly a leaf concern, the dividing line between subscreen composition and command lists.

`output` participates fully in [interpolation](#org946fa96), including both `$` (global vars) and `@` (params) sigils.
Allowing `@` in `output` lets a composite `Screen's` caller determine where a file is written (eg. a directory name).


<a id="orgde1259e"></a>

### Buckets

An output file may declare one or more named `buckets`: non-contiguous regions of the final file, assembled in a fixed, declared order.
This lets a leaf whose single responsibility requires code in several places in the underlying syntax do so without that being mistaken for multiple responsibilities.

A leaf targets a bucket in one of two ways:

-   *Single-bucket form* (the common case): `commands` stays a flat list, with an optional `bucket` field naming which bucket it targets.

An omitted `bucket` defaults to the first bucket declared for that output file.

-   *Multi-bucket form*: `commands` becomes a map from bucket name to that bucket&rsquo;s lines, letting one leaf contribute to several buckets at once.

`bucket` is not used in this form; the map&rsquo;s keys are the assignment.

    outputs:
      - path: CMakeLists.txt
        buckets: [header, project_setup, targets, linking]
      - path: src/CMakeLists.txt
        buckets: [sources]

    screens:
      target_sources:
        output: src/CMakeLists.txt
        bucket: sources
        commands:
          - add_library(core_sources OBJECT @sources)
    
      library:
        output: CMakeLists.txt
        commands:
          header:
            - include(GNUInstallDirs)
          targets:
            - add_library(mylib STATIC ...)
          linking:
            - target_link_libraries(mylib PUBLIC ...)

If an output file declares no `buckets` list at all, it implicitly has exactly one bucket, named `default`; this preserves the single-block behavior with no migration for simple files.
Buckets are scoped to the file that declares them; two output files may reuse the same bucket name independently.
Bucket names are author-declared only and do not participate in [interpolation](#org946fa96).


<a id="orgf1e79e0"></a>

### Output Validation

-   An `output` containing no sigils is *literal* and is checked against the top-matter `outputs` list (typo-catching), including that any declared `bucket` is a member of that output&rsquo;s declared bucket set.
-   An `output` containing any sigil (`$` or `@`) cannot be resolved statically; global vars are themselves mutable during execution, so even a `$`-only path cannot be assumed known at file-parse time.

Such an `output` is automatically exempted from the `outputs` check; sigil presence is mechanically detectable, so no per-screen override field is needed.

-   If the top-matter sets `disallow_interpolated_paths: true`, any sigil found in an `output` is a hard schema/lint error instead of being exempted; a project-wide style lock.
-   If `output` is omitted entirely on a leaf, it defaults to the top-matter `default_output`.
-   Multiple leaves may target the same `(output, bucket)` pair.

Their emitted content is concatenated in traversal order; see [6](#org7093327).
ZyBS performs no semantic conflict detection between leaves sharing a bucket; avoiding conflicting build-system code is the author&rsquo;s responsibility.


<a id="org62b0148"></a>

## Directory Dependencies

A leaf `Screen` declares `uses_directories`: the list of top-matter `directories` entries its `commands` actually depend on, whether directly (a path alias) or via a discovery-derived var.
An empty list is a meaningful, explicit statement that a `Screen` makes no assumption about project layout and is fully relocatable.

`uses_directories` is lint-checked against actual usage inside `commands`.
A declared dependency that is not referenced, or a reference not covered by the declaration, is a hard error, not a warning.

A composite `Screen` never declares `uses_directories` itself.
Its effective directory dependency is computed as the union of everything reachable beneath it in the graph, the same reasoning that keeps [2](#org425f280) membership and structural depth from needing separate, hand-maintained bookkeeping.
This gives tooling an always-accurate answer, at any level of the pyramid, to what parts of the project layout peeling a `Screen` touches.
See [5](#org3c2260f) for how `directories` itself is declared.


<a id="org946fa96"></a>

# Interpolation Grammar

`Screen` `commands` and `output` strings are parsed through a small interpolation grammar before being handed to the underlying build system, and, for `build_system_cmd`, before shell expansion.


<a id="orga2c5151"></a>

## Sigils

Two sigils are recognized:

-   `$name`: resolves against the global `vars` map in whatever state it currently holds, which may have been mutated by an earlier-executed `Screen`.
-   `@name`: resolves against the calling `Screen's` own bound params, as passed at that specific call site; frozen at bind time, unaffected by later global mutation.

Both sigil characters may be overridden per file in top-matter (`global_sigil`, `arg_sigil`; defaults `$` and `@`), in case a target build system itself relies heavily on `$` or `@`.

    global_sigil: "$"   # default
    arg_sigil: "@"      # default


<a id="org23bf2d3"></a>

## Escaping

A doubled sigil produces a literal instance of that character: `$$` → literal `$`, `@@` → literal `@`.
This applies regardless of any `global_sigil~/~arg_sigil` override; doubling whatever sigil is configured escapes it.


<a id="orgc428bbe"></a>

## Formatting

When a var or arg resolves to a list, it is joined into a single string.
The default join separator is a single space, unless overridden per file:

    default_sep: " "   # default

A separator may also be overridden at a specific interpolation site:

    ${cxx_flags:,}   # comma-joined at this site only
    @sources         # space-joined (or default_sep); no override given

Formatting overrides apply identically to both `$` and `@` sigiled references.


<a id="org3c2260f"></a>

# Directory Discovery

A `.zyl` file may declare a default directory structure in top-matter, used both for path organization and for automatically discovering files that feed into higher-level `Screens` without an author hand-listing them.


<a id="orgca7e6c3"></a>

## Declaring Directories

    directories:
      build_dir: build                              # plain path alias
      src: ["$sources:*.cpp", "$headers:*.h|*.hpp"]
      include: ["$headers:*.h|*.hpp"]

A `directories` entry is either:

-   A plain string: a path alias, merged directly into the global `vars` namespace (eg. `$build_dir` resolves to `"build"`).

Behaves exactly like any other global var, including being relocatable further down the tree via a `$`-sigiled param rebind.

-   A list of scan rules of the form `"$varname:pattern[|pattern...]"`.

Each rule scans that directory for files matching the given pattern(s) (multiple patterns separated by `|`) and appends matches into the named derived var.
Multiple directories may feed the same derived var; results are concatenated in the order the directories are declared, then in lexical filename order within each directory.

Discovery-derived vars are resolved once, upfront, before any `Screen` executes; not lazily, and not re-triggered if a directory is later relocated by a `$`-sigiled param rebind further down the tree.
This trades away dynamic re-scanning for predictable, inspectable output.
A directory scan that matches zero files is a hard error by default.


<a id="org49ea377"></a>

## The `$discover` Builtin

Because a directory&rsquo;s own path may itself be interpolated (eg. a composite `Screen's` caller relocates it via a `$`-sigiled param), the upfront top-matter scan cannot always resolve a final path at parse time.
For that ambiguous case the interpolation grammar provides one builtin function call:

    commands:
      - add_library(extra_sources OBJECT $discover($src))            # reuses $src's declared pattern(s)
      - add_library(extra_sources OBJECT $discover($src, "*.cc"))    # explicit pattern override

`discover` is a reserved word: no author-defined global var or param may be named `discover`, enforced as a hard schema/lint error on collision.
Unlike top-matter directory scanning, a `$discover` call resolves at the point it is emitted, against whatever the referenced directory currently resolves to at that point in traversal.

A discovery-derived var referenced inside a `Screen` sitting beneath a composite that has rebound the relevant directory&rsquo;s path is a hard lint error unless that reference goes through `$discover` instead.
This prevents a `Screen` from silently using a stale, upfront-resolved file list in a subtree where the directory it was scanned from has since moved.
See [3.5](#org62b0148) for how a `Screen` declares which directories it depends on.


<a id="org7093327"></a>

# Execution Semantics

The `Screen` hierarchy is executed *depth-first*: a composite `Screen's` `Subscreens` are fully resolved and emitted before the composite&rsquo;s own `commands` (composites never carry `commands`, but the rule generalizes the ordering guarantee).
Where a composite calls multiple `Subscreens`, they are emitted strictly in the order listed; order may matter to the underlying build system, and no reordering or parallelization is performed.
See [12](#org7b2eac1).


<a id="org9634628"></a>

## Output Assembly

Because leaves may target different `output` files and `buckets`, emission is depth-first per `(output file, bucket)`.
As the tree is walked, each leaf&rsquo;s `commands` are appended to the in-progress content of whichever bucket its `output~/~bucket` (or, in multi-bucket form, each map key) names, in traversal order.
A single traversal may therefore be assembling many bucket buffers, across several output files, concurrently.
Once traversal completes, each output file&rsquo;s final content is produced by concatenating its buckets in the order they were declared in `outputs`.


<a id="orgc280c40"></a>

## Global Mutation

Binding a `$`-sigiled param does not just supply a default; it *writes* to the global `vars` map for the remainder of execution, visible to every subsequently-executed `Screen`, regardless of whether that later `Screen` was called by the same parent or an unrelated one.
This is intentional shared mutable state, the mechanism by which one `Screen's` choices (eg. setting compiler flags) can be felt by later, structurally unrelated `Screens`.

Because this is order-dependent, sibling `Screens` that both declare a `$`-sigiled param of the same name may silently affect one another depending on call order.
This is a real footgun class worth linting for; see [12](#org7b2eac1).


<a id="org524a0d0"></a>

# `.zyl` File

A `.zyl` file defines the build abstraction for a specific build system: its `Screens`, their commands, and how they assemble into real build-system output.
Generally each build system has one `.zyl` file (eg. `CMake.zyl` or `GNUMake.zyl`).
A `.zyl` file is not invoked directly; a project&rsquo;s [8](#orgefe5a8e) selects one entry `Screen` from it and supplies that `Screen's` arguments.
A `.zyl` file must be fully usable this way with no companion `.zydoc` file present.
When present, a `.zydoc` shares the `.zyl's` base name.


<a id="orge58d847"></a>

## Top-Matter

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Type</th>
<th scope="col" class="org-left">Required</th>
<th scope="col" class="org-left">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>version</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">ZyBS spec version this file targets.</td>
</tr>

<tr>
<td class="org-left"><code>build_system</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">Display name of the underlying build system.</td>
</tr>

<tr>
<td class="org-left"><code>build_system_cmd</code></td>
<td class="org-left">list of string</td>
<td class="org-left">yes</td>
<td class="org-left">Literal commands invoked to run the build.</td>
</tr>

<tr>
<td class="org-left"><code>description</code></td>
<td class="org-left">string (block)</td>
<td class="org-left">no</td>
<td class="org-left">Prose description of what this file builds.</td>
</tr>

<tr>
<td class="org-left"><code>global_sigil</code></td>
<td class="org-left">string</td>
<td class="org-left">no</td>
<td class="org-left">Override for the global-var sigil; default <code>$</code>.</td>
</tr>

<tr>
<td class="org-left"><code>arg_sigil</code></td>
<td class="org-left">string</td>
<td class="org-left">no</td>
<td class="org-left">Override for the param sigil; default <code>@</code>.</td>
</tr>

<tr>
<td class="org-left"><code>default_sep</code></td>
<td class="org-left">string</td>
<td class="org-left">no</td>
<td class="org-left">Override for the default list-join separator; default space.</td>
</tr>

<tr>
<td class="org-left"><code>vars</code></td>
<td class="org-left">map</td>
<td class="org-left">no</td>
<td class="org-left">Global variable map, mutable during execution.</td>
</tr>

<tr>
<td class="org-left"><code>directories</code></td>
<td class="org-left">map</td>
<td class="org-left">no</td>
<td class="org-left">Default directory structure and discovery rules; see <a href="#org3c2260f">5</a>.</td>
</tr>

<tr>
<td class="org-left"><code>outputs</code></td>
<td class="org-left">list of object</td>
<td class="org-left">no</td>
<td class="org-left">Declared output files, each with a <code>path</code> and optional <code>buckets</code> list, for typo-catching.</td>
</tr>

<tr>
<td class="org-left"><code>default_output</code></td>
<td class="org-left">string</td>
<td class="org-left">no</td>
<td class="org-left">Output file a leaf uses if it omits <code>output</code>.</td>
</tr>

<tr>
<td class="org-left"><code>disallow_interpolated_paths</code></td>
<td class="org-left">boolean</td>
<td class="org-left">no</td>
<td class="org-left">If true, any sigil found in a leaf&rsquo;s <code>output</code> is a hard error; default false.</td>
</tr>

<tr>
<td class="org-left"><code>screens</code></td>
<td class="org-left">map</td>
<td class="org-left">yes</td>
<td class="org-left">The screen registry; see <a href="#org490d75f">3</a>.</td>
</tr>
</tbody>
</table>


<a id="orgec9fa2a"></a>

## Format

    version: 0.0.1
    build_system: CMake
    build_system_cmd:
      - cmake -S .
    -B $build_dir
      - cmake --build $build_dir
    
    description: |
      builds the thing
      yada yada
    
    global_sigil: "$"
    arg_sigil: "@"
    default_sep: " "
    disallow_interpolated_paths: false
    
    outputs:
      - path: CMakeLists.txt
        buckets: [header, project_setup, targets, linking]
      - path: src/CMakeLists.txt
        buckets: [sources]
      - path: CMakePresets.json
    default_output: CMakeLists.txt
    
    directories:
      build_dir: build
      src: ["$sources:*.cpp", "$headers:*.h|*.hpp"]
      include: ["$headers:*.h|*.hpp"]
    
    vars:
      cxx_flags: [-Wall, -Wextra, -O2]
    
    screens:
      root_project_setup:
        output: CMakeLists.txt
        bucket: header
        uses_directories: []
        commands:
          - cmake_minimum_required(VERSION 3.28)
          - project(myapp)
    
      target_sources:
        params: [sources]
        output: src/CMakeLists.txt
        bucket: sources
        uses_directories: [src]
        commands:
          - add_library(core_sources OBJECT @sources)
    
      library:
        params: [lib_name]
        subscreens:
          - screen: target_sources
            args:
              sources: [src/foo.cpp, src/bar.cpp]
        commands:
          - add_library(@lib_name STATIC ...)
    
      binary:
        params: [bin_name]
        subscreens:
          - screen: target_sources
            args:
              sources: ${build_dir}/main.cpp
        commands:
          - add_executable(@bin_name ...)


<a id="org1201310"></a>

## Screen Fields

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Type</th>
<th scope="col" class="org-left">Required</th>
<th scope="col" class="org-left">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>params</code></td>
<td class="org-left">list of string</td>
<td class="org-left">no</td>
<td class="org-left">Declared parameters. May be <code>$</code>-sigiled to auto-bind to a global var.</td>
</tr>

<tr>
<td class="org-left"><code>commands</code></td>
<td class="org-left">list of string, or map of string to list of string</td>
<td class="org-left">leaves</td>
<td class="org-left">Literal build-system code. Map form assigns per bucket; list form uses <code>bucket</code>. Mutually exclusive with <code>subscreens</code>.</td>
</tr>

<tr>
<td class="org-left"><code>output</code></td>
<td class="org-left">string</td>
<td class="org-left">leaves</td>
<td class="org-left">Output file this leaf&rsquo;s <code>commands</code> are written to.</td>
</tr>

<tr>
<td class="org-left"><code>bucket</code></td>
<td class="org-left">string</td>
<td class="org-left">no</td>
<td class="org-left">Which bucket of <code>output</code> this leaf targets (list form only). Defaults to the first declared bucket.</td>
</tr>

<tr>
<td class="org-left"><code>uses_directories</code></td>
<td class="org-left">list of string</td>
<td class="org-left">leaves</td>
<td class="org-left">Which top-matter <code>directories</code> this leaf depends on. Never declared on composites; see <a href="#org62b0148">3.5</a>.</td>
</tr>

<tr>
<td class="org-left"><code>subscreens</code></td>
<td class="org-left">list of object</td>
<td class="org-left">composites</td>
<td class="org-left">Calls into other <code>Screens</code>. Mutually exclusive with <code>commands~/~output~/~bucket~/~uses_directories</code>.</td>
</tr>
</tbody>
</table>

Each entry in `subscreens` has the shape:

    - screen: <screen name>
      args:
        <param name>: <literal value, or $/@ interpolated string>


<a id="orgefe5a8e"></a>

# Zyfile

A `Zyfile` is the user-written, per-project build file.
It names which `.zyl` file to use and invokes one or more `Screens` as function-style calls, supplying their arguments directly.
It does not define new `Screens`, and it does not override top-matter (vars, sigils, outputs, etc.); anything beyond screen invocation belongs in the `.zyl` file.
A `Zyfile` is not YAML; it uses its own small custom grammar, purpose-built to read like an invocation rather than a config file.


<a id="org0f01231"></a>

## Grammar

A `Zyfile` is a sequence of function-style calls, one per line:

    zyfile_version(0.0.1)
    zyl(./CMake.zyl)
    
    binary(bin_name: myapp)
    library(lib_name: mylib, sources: foo.cpp bar.cpp)

-   `zyfile_version(<version>)`: required, first line.

Pins the `Zyfile` grammar version this file was written against.

-   `zyl(<path>)`: required, exactly once.

Path to the `.zyl` file this `Zyfile` invokes.

-   A call whose name matches a `Screen` in the target `.zyl` binds that `Screen's` `params` via its keyword arguments.

The first `Screen` call encountered is the entry point, the root of traversal, with no separate directive needed to name it.
Every subsequent `Screen` call supplies args for a `Screen` the user has peeled back; see [9](#org8d99f8a).

-   Arguments are comma-separated `key: value` pairs inside the call&rsquo;s parentheses.
    -   A bare, unquoted value with no top-level comma is split on whitespace: one token is a scalar, several form a space-separated list.
    -   A bracketed, comma-separated value is always a list; needed whenever list items must coexist with other `key: value` pairs in the same call, since the brackets protect the internal commas from being read as pair separators.
    -   A value containing a literal space that must stay one scalar is wrapped in double quotes.
    -   `#` starts a line comment, to end of line.

Full raw-string support for values containing unescaped quotes, backslashes, or newlines is a deferred stretch goal; see [12](#org7b2eac1).


<a id="orgcd025e0"></a>

## Naming

When a project has only one build system, its `Zyfile` is named plainly: `Zyfile`.
When a project has more than one `.zyl` present, each `Zyfile` is named `Zyfile.<build_system>` (eg. `Zyfile.CMake`, `Zyfile.Bazel`); the reverse of the `.zyl=/`.zydoc=/=.zystate= naming order, since a `Zyfile's` own identity comes first and the build system is a qualifier on it.
See [11](#org840fbc6) for how `-b` resolves which `Zyfile` a command targets.


<a id="org21341bb"></a>

## Validation and Peeling

A `Zyfile` is validated against whichever `.zystate` currently applies for its build system (project-scoped if present, otherwise system-scoped).
Every `Screen` name in that `.zystate's` `active_screens` must have a corresponding call in the `Zyfile`, in addition to the always-required entry call.
A missing call is a hard build failure; the error names exactly which `Screen` needs a call and what params it expects, seeded with the value the `.zyl` currently supplies for it internally where one exists.

A call present for a `Screen` no longer listed in `active_screens` (the user has since run `zybs return` on it) is a lint warning, not an error; the file is left untouched for the user to clean up at their own pace.

No tool ever writes to a `Zyfile`.
`zybs peel` and `zybs return` only ever modify `.zystate`; see [9](#org8d99f8a).
This is deliberate: the still-failing build&rsquo;s guiding error message is the entire teaching mechanism, and auto-editing the `Zyfile` would remove the moment the learning happens.


<a id="org8d99f8a"></a>

# .zystate

A `<build_system>.zystate` file tracks which `Screens` are currently active (peeled back) for a given build system, independently of the `Zyfile` itself.
It exists at two possible scopes:

-   *System-scoped*: lives with the user, not the project, keyed per build system.

Expresses that user&rsquo;s personal default: how peeled they generally like starting a new project of this kind.

-   *Project-scoped*: lives with the project.

When present for a given build system, it entirely replaces the system-scoped file for that build system in that project; no merging.

    # eg. ~/.config/zybs/CMake.zystate (system-scoped)
    version: 0.0.1
    active_screens: [library, binary]

    # eg. ./CMake.zystate (project-scoped)
    version: 0.0.1
    active_screens: [library, binary]


<a id="org78e7a74"></a>

## Fields

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Type</th>
<th scope="col" class="org-left">Required</th>
<th scope="col" class="org-left">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>version</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">ZyBS spec version this <code>.zystate</code> targets.</td>
</tr>

<tr>
<td class="org-left"><code>active_screens</code></td>
<td class="org-left">list of string</td>
<td class="org-left">yes</td>
<td class="org-left"><code>Screens</code> currently peeled back for this build system.</td>
</tr>
</tbody>
</table>


<a id="org2617583"></a>

## `zybs peel` and `zybs return`

`zybs peel <screen>` adds a `Screen` name to the project-scoped `.zystate's` `active_screens`, creating the file if absent.
`zybs return <screen>` removes it.
Neither command ever touches the `Zyfile`; see [8.3](#org21341bb).


<a id="org9c3253d"></a>

## Shared Screens (DAG) Caveat

Because `Screens` form a DAG rather than a strict tree, a single peeled `Screen` name may be reachable from more than one parent (eg. `target_sources` called from both `library` and `binary`).
Peeling is uniform by name: one `Zyfile` call for that `Screen` name supplies its args regardless of which parent would otherwise have called it.
Path-scoped peeling, letting the same shared `Screen` be peeled differently depending on which parent reaches it, is a deferred question; see [12](#org7b2eac1).


<a id="org0ac67a9"></a>

# `.zydoc` File

A `.zydoc` file (extension matching its paired `.zyl` file; eg. `CMake.zyl` alongside `CMake.zydoc`) is the companion pedagogical document to a `.zyl` file.
It carries everything a learner-facing GUI/CLI needs to describe `Screens` and `Modules` to a novice; what a `Screen` does, what taking it on means, and how `Screens` group into a learning progression; with zero effect on how a build executes.

A `.zydoc` file is purely additive.
It must declare which `.zyl` file it documents, and tooling treats a `.zydoc` referencing a `.zyl` file of a different `version` as stale, warning the maintainer that documentation may be out of date.


<a id="orgb037b48"></a>

## Format

    version: 0.0.1
    target: ./CMake.zyl
    target_version: 0.0.1   # version of the .zyl file this .zydoc file was written against
    
    modules:
      - id: fundamentals
        order: 0
        title: "What actually becomes code"
        description: >
          Screens here decide what source files exist and how they're grouped.
        screens: [target_sources]
    
      - id: assembly
        order: 1
        title: "Turning code into something runnable"
        description: >
          Screens here decide how compiled pieces become libraries or executables.
        screens: [library, binary]
    
    screen_docs:
      target_sources:
        description: >
          Compiles your source files into reusable object code.
        responsibility: >
          Listing every source file explicitly, and understanding what an
          object library is versus a linked one.
    
      library:
        description: >
          Packages compiled objects into a linkable library.
        responsibility: >
          Deciding how compiled objects are packaged into a linkable library.
    
      binary:
        description: >
          Produces the runnable executable.
        responsibility: >
          Understanding how object code gets linked into a runnable executable.


<a id="org3b09900"></a>

## Top-Matter

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Type</th>
<th scope="col" class="org-left">Required</th>
<th scope="col" class="org-left">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>version</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">ZyBS spec version this <code>.zydoc</code> file targets.</td>
</tr>

<tr>
<td class="org-left"><code>target</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left">Path to the <code>.zyl</code> file this <code>.zydoc</code> file documents.</td>
</tr>

<tr>
<td class="org-left"><code>target_version</code></td>
<td class="org-left">string</td>
<td class="org-left">yes</td>
<td class="org-left"><code>version</code> of the target <code>.zyl</code> file, for staleness detection.</td>
</tr>

<tr>
<td class="org-left"><code>modules</code></td>
<td class="org-left">list</td>
<td class="org-left">yes</td>
<td class="org-left">See <a href="#org425f280">2</a>.</td>
</tr>

<tr>
<td class="org-left"><code>screen_docs</code></td>
<td class="org-left">map</td>
<td class="org-left">yes</td>
<td class="org-left">Per-<code>Screen</code> pedagogical content, keyed by <code>Screen</code> name.</td>
</tr>
</tbody>
</table>


<a id="orgf934d85"></a>

## Screen Documentation Fields

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">Field</th>
<th scope="col" class="org-left">Type</th>
<th scope="col" class="org-left">Required</th>
<th scope="col" class="org-left">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-left"><code>description</code></td>
<td class="org-left">string (block)</td>
<td class="org-left">yes</td>
<td class="org-left">What the <code>Screen</code> does, in learner-facing terms.</td>
</tr>

<tr>
<td class="org-left"><code>responsibility</code></td>
<td class="org-left">string (block)</td>
<td class="org-left">yes</td>
<td class="org-left">What a learner takes on themselves by peeling this <code>Screen</code> back.</td>
</tr>
</tbody>
</table>

Every `Screen` in the target `.zyl` file should have a corresponding `screen_docs` entry.
A `Screen` with no entry is not a schema error (the `.zydoc` may simply be incomplete) but should be flagged by tooling as a warning.


<a id="org840fbc6"></a>

# Command Line Interface

Both the CLI and a GUI are clients of one common API.
The GUI does not wrap or shell out to the CLI, and the CLI contains no logic the API itself lacks.
This is the mechanism behind &ldquo;no feature of the GUI is impossible with the CLI&rdquo;: it holds because both are built on the same underlying operation set.
Both are expected to remain equally capable at every operation below.


<a id="orgfa2dfec"></a>

## Global Flags

-   `-b`, `--build-system <name>`: selects which build system&rsquo;s `.zyl=/`.zydoc=/=.zystate=/=Zyfile= a command operates against; needed whenever more than one is present in a project.

Resolution, based on how many `Zyfiles` are present and whether `-b` is given:

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-right" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-right">Zyfiles found</th>
<th scope="col" class="org-left"><code>-b</code> given</th>
<th scope="col" class="org-left">Result</th>
</tr>
</thead>
<tbody>
<tr>
<td class="org-right">1</td>
<td class="org-left">no</td>
<td class="org-left">use it</td>
</tr>

<tr>
<td class="org-right">1</td>
<td class="org-left">yes, matches</td>
<td class="org-left">use it</td>
</tr>

<tr>
<td class="org-right">1</td>
<td class="org-left">yes, doesn&rsquo;t match</td>
<td class="org-left">error</td>
</tr>

<tr>
<td class="org-right">2+</td>
<td class="org-left">no</td>
<td class="org-left">error (ambiguous)</td>
</tr>

<tr>
<td class="org-right">2+</td>
<td class="org-left">yes, matches one</td>
<td class="org-left">use that one</td>
</tr>

<tr>
<td class="org-right">2+</td>
<td class="org-left">yes, matches none</td>
<td class="org-left">error</td>
</tr>
</tbody>
</table>


<a id="org4f28fa6"></a>

## Project Lifecycle

-   `zybs init`: scaffold a new project; pick/confirm a `.zyl`, write a minimal starting `Zyfile` with a first call for the `.zyl's` designated default/simplest entry `Screen`.
-   `zybs build`: full pipeline; validate, resolve `.zystate`, generate output files, invoke `build_system_cmd`.
-   `zybs generate`: same as `build` minus the final invocation; writes output files for inspection without building.
    -   `--output <file>`: restrict generation to a single declared output file.
    -   `--debug`: prints every global var, string expansion, directory discovery, subscreen call, and output write as it happens, in a tagged, grep-friendly format; see [11.7](#org3f5d73d).
    -   `--debug-filter <tags>`: narrows `--debug` output to a comma-separated list of tags.


<a id="orgbc98049"></a>

## Screen State

-   `zybs peel <screen>`: see [9](#org8d99f8a).
-   `zybs return <screen>`: see [9](#org8d99f8a).


<a id="org1b8dd0d"></a>

## Inspection

-   `zybs tree`: renders the pyramid in the terminal, marking active vs. collapsed `Screens` and their `Modules`.
-   `zybs screens`: flat list of every `Screen` in the `.zyl`, with module, one-line description, and active state.
-   `zybs show <screen>`: full detail; description, responsibility, params, `uses_directories`, output/bucket, current active state.
    -   `--resolved`: prints the fully substituted command text (vars/args/discovery all resolved) rather than the raw template.
-   `zybs modules`: lists modules in order, each with its `Screens` and, if a current project exists, distance from the user&rsquo;s current active set.
-   `zybs which-directories <screen>`: prints a composite `Screen's` computed (unioned) `uses_directories`, since that value is derived, not authored; see [3.5](#org62b0148).


<a id="org882f0d0"></a>

## Validation

-   `zybs validate`: runs every lint/schema rule across `.zyl=/`.zydoc=/=Zyfile=/=.zystate= without building anything.
    -   `--fix`: applies only mechanically-safe fixes (a stale `.zydoc` `target_version`, `outputs~/~buckets` ordering, a missing `screen_docs` stub).

Never writes to a `Zyfile`, consistent with [8.3](#org21341bb).

-   `--strict`: treats warnings (an unused peel call, incomplete `screen_docs`, etc.) as hard failures.
-   `--zyl-only` / `--zydoc-only`: scopes a validation pass to one file.


<a id="org7ebf8b9"></a>

## Authoring

-   `zybs new screen <name>`: interactive wizard that scaffolds a new `Screen` stub directly into the `.zyl` (leaf or composite, `output~/~bucket`, `uses_directories`, `params`).
-   `zybs new zydoc-entry <screen>`: scaffolds a `screen_docs` stub (with placeholder text) for a `Screen` `validate` flagged as undocumented.
-   `zybs fmt`: canonicalizes a `.zyl=/`.zydoc&rsquo;s= YAML formatting.
-   `zybs completions <shell>`: generates shell completions.


<a id="org3f5d73d"></a>

## Debug Output

`zybs generate --debug` prints one tagged line per event, so a category can be isolated with a plain `grep` or narrowed with `--debug-filter`:

-   `[VAR]`: initial state of every global var and resolved directory alias, printed once up front.
-   `[DISCOVER]`: a directory scan firing (top-matter or `$discover`), with match count and files.
-   `[CALL]`: entering a subscreen, with its bound args, in traversal order; the depth-first walk made visible.
-   `[EXPAND]`: a single `$~/~@` resolution at a specific interpolation site.
-   `[MUTATE]`: a `$`-sigiled param write changing a global var, with before/after value and the call responsible.

The primary tool for diagnosing the sibling global-mutation footgun in [6.2](#orgc280c40).

-   `[WRITE]`: a leaf appending resolved text to a specific `(output, bucket)` buffer.
-   `[ASSEMBLE]`: the final per-file bucket concatenation, once traversal completes.

    [VAR]      build_dir = ./build
    [DISCOVER] src -> sources: matched 2 files (foo.cpp, bar.cpp)
    [CALL]     binary -> target_sources  args={sources: ./build/main.cpp}
    [EXPAND]   @sources -> "./build/main.cpp"          (site: target_sources.commands[0])
    [MUTATE]   cxx_flags: [-Wall, -Wextra, -O2] -> [-O3]   (via binary's $cxx_flags param)
    [WRITE]    src/CMakeLists.txt:sources <- "add_library(core_sources OBJECT ./build/main.cpp)"
    [ASSEMBLE] CMakeLists.txt <- header, project_setup, targets, linking


<a id="org7b2eac1"></a>

# Future Considerations

Known open questions, deliberately deferred rather than resolved, and not part of the current spec:

-   *Explicit subscreen ordering independent of list order*: subscreens currently execute strictly in list order.

If execution is ever parallelized, an explicit ordering/dependency mechanism will be needed to preserve correctness where build-system order matters.

-   *Readiness gating threshold*: whether the caution for peeling a `Screen` far below the learner&rsquo;s current `Module` is configurable per file (eg. a top-matter `readiness_gate_threshold`) or fixed by the tool itself.
-   *Command-length / responsibility linting*: a soft lint nudging leaf `Screens` with unusually large or multi-purpose `commands` toward decomposition into `subscreens`.
-   *Naming smell linting*: flagging `Screen` names containing conjunctions (eg. `_and_`) as a signal that a `Screen` may cover more than one responsibility.
-   *Sibling global-mutation collisions*: a possible lint warning when two `Screens` likely to be executed in sequence both declare a `$`-sigiled param of the same name.
-   *Raw strings in Zyfile values*: CMake-style `R"(...)"` syntax for values containing unescaped quotes, backslashes, or newlines.
-   *Path-scoped peeling*: allowing a shared (DAG) subscreen to be peeled with different args depending on which parent reaches it, rather than the current uniform-by-name assumption.

