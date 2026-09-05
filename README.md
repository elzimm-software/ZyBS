
# Table of Contents

1.  [ZyBS (pronounced &ldquo;Zeebs&rdquo;)](#orga5d19f4)
    1.  [The Problem](#org21341bb)
    2.  [Philosophy](#org840fbc6)
    3.  [Who It&rsquo;s For](#org5ee4105)
    4.  [How It Works, In Practice](#org524a0d0)
    5.  [Key Features](#orgefe5a8e)
    6.  [In Short](#org0ac67a9)



<a id="orga5d19f4"></a>

# ZyBS (pronounced &ldquo;Zeebs&rdquo;)

A build system abstraction framework for teaching, not just building.


<a id="org21341bb"></a>

## The Problem

Build systems are complicated because they have to be.
Real-world software and hardware are varied and shifting targets, and reproducible builds require tools that can handle that variety.
But those same tools; CMake, Make, Bazel, and the rest; are brutal for beginners.
A novice doesn&rsquo;t need to understand fifteen years of accumulated build-system complexity just to compile a &ldquo;hello world.&rdquo;

Most tools force a binary choice: give learners a black-box wrapper that hides everything (and teaches nothing), or throw them straight into the deep end of the real tool (and lose them).
ZyBS exists to avoid that choice.


<a id="org840fbc6"></a>

## Philosophy

ZyBS lets an experienced developer collapse an entire, complex build down to **one command**, while letting a novice **peel back that abstraction one layer at a time**; exposing exactly as much complexity as they&rsquo;re ready for, and no more.

The core idea: every layer a learner peels back has a named, defined responsibility they are now taking on themselves.
At any point, a learner should know precisely what new thing they&rsquo;re on the hook for; never more, never less.
Learning to use a build system becomes a gradual, legible process instead of an all-or-nothing leap.

This philosophy shows up throughout the design:

-   Abstraction is layered, not flattened; you can zoom in exactly as far as you want.
-   Every unit of abstraction has one job.

Nothing is a grab-bag of unrelated behavior.

-   The system never auto-fixes a learner&rsquo;s mistakes on their behalf; a failing build with a clear, specific error message *is* the teaching moment, and ZyBS is careful not to paper over it.
-   Documentation and pedagogy are optional, additive, and separate from the mechanics of building.

You can run ZyBS with zero teaching material present; the teaching layer is there for when you want it.


<a id="org5ee4105"></a>

## Who It&rsquo;s For

-   **Experienced developers / maintainers** who want to package up a build system (CMake, Make, Bazel, whatever) into a reusable, teachable abstraction for their team, students, or open-source contributors.
-   **Novices and learners** who want to build real projects with real tools, without needing to understand the entire build system on day one; and who want a guided, structured path toward eventually understanding it.
-   **Teams** who want new contributors to become self-sufficient with the actual build tooling over time, rather than staying permanently dependent on a hand-maintained wrapper script.


<a id="org524a0d0"></a>

## How It Works, In Practice

At a high level, using ZyBS involves three kinds of files, each with a different author:

-   An expert writes the abstraction; the full hierarchy of what can be peeled back, from &ldquo;one command builds everything&rdquo; down to raw build-system code.
-   A learner (or any user) writes a small per-project file that just picks an entry point and supplies a few values; no build-system knowledge required to get started.
-   An expert can optionally write teaching material; plain-language descriptions of what each layer does and why, grouped into a learning progression; with zero effect on how anything actually builds.

A project can run with just the abstraction and the per-project file; the teaching material is always optional.

As a learner grows more comfortable, they &ldquo;peel back&rdquo; layers one at a time using simple commands, taking on a bit more responsibility and exposure to the real build system each time.
Peeling back a layer well ahead of where you&rsquo;ve been working triggers a gentle caution; a nudge that you might be touching something you&rsquo;re not quite ready for yet, not a hard block.


<a id="orgefe5a8e"></a>

## Key Features

-   **Progressive disclosure of complexity.** Collapse a build to one command, or expose it piece by piece; your choice, at any granularity.
-   **Guided learning structure.** Related concepts are grouped into named, ordered lessons independent of the underlying technical structure, so the learning path can be organized around what&rsquo;s conceptually easy, not just what&rsquo;s structurally shallow.
-   **Real build systems underneath.** ZyBS is a teaching and organizing layer over the genuine build system; not a toy replacement for it.

What you learn transfers directly.

-   **Reusable, composable building blocks.** Common pieces of build logic can be shared and reused across many higher-level build targets.
-   **Deliberately minimal per-project files.** The file a project author actually maintains day-to-day is small, readable, and purpose-built; not a sprawling config format.
-   **Personal defaults.** Your own preferred &ldquo;starting depth&rdquo; for a given build system can be remembered and reused across projects.
-   **No silent auto-fixing of the learner&rsquo;s file.** Tooling will tell you exactly what&rsquo;s missing and why; but it won&rsquo;t edit your project file behind your back.

The friction is the point.

-   **One tool, two faces.** The command line and the GUI are built on the exact same underlying operations, so nothing is CLI-only or GUI-only; pick whichever interface fits how you like to work.
-   **Rich inspection and debugging tools.** See the full build pyramid at a glance, inspect any single piece in detail, or trace a build&rsquo;s execution step-by-step when something isn&rsquo;t behaving the way you expect.
-   **Validation with helpful errors.** Problems are caught early, with error messages designed to teach rather than just fail.


<a id="org0ac67a9"></a>

## In Short

ZyBS isn&rsquo;t trying to replace your build system; it&rsquo;s trying to make the *on-ramp* to your build system humane.
It gives experts a way to encode not just &ldquo;how to build this,&rdquo; but &ldquo;how to learn to build this,&rdquo; so that the path from zero knowledge to full mastery is something a beginner can actually walk, one clearly-labeled step at a time.

