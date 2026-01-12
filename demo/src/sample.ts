export const SAMPLE_MARKDOWN = `# Hong Minhee's Markdown style convention

This document describes the Markdown style convention enforced by Hongdown.

## Philosophy

The core principle of this style is:

> _Markdown should be readable as plain text, not just after rendering._

A well-formatted Markdown document should convey its structure and emphasis
clearly even when viewed in a plain text editor without any rendering. You
shouldn't need to render the document to HTML to understand its formatting and
hierarchy.

This philosophy leads to several practical implications:

- _Visual structure in source_: Headings, lists, and sections should be visually
  distinct in the raw text.
- _Consistent spacing_: Predictable whitespace patterns help readers scan the
  document structure.
- _Minimal escaping_: Choose delimiter styles that minimize the need for escape
  characters.
- _Reference-style links_: Keep prose readable by moving URLs out of the text
  flow.

This style prioritizes reading over writing. Many rules are tedious to follow
manually—and that's intentional. The assumption is that you'll use an automated
formatter like [Hongdown] to handle the mechanical work, freeing you to focus on
content rather than formatting details.

[Hongdown]: https://github.com/dahlia/hongdown

## Headings

### Setext-style for top-level headings

Use Setext-style (underlined) headings for document titles (H1) and major
sections (H2):

\`\`\`markdown
# Document Title

## Section Name
\`\`\`

_Rationale_: Setext headings create strong visual separation in plain text. The
underline makes the heading immediately recognizable without needing to count
\`#\` characters.

### ATX-style for subsections

Use ATX-style (\`###\`, \`####\`, etc.) for subsections within a section:

\`\`\`markdown
### Subsection

#### Sub-subsection
\`\`\`

_Rationale_: ATX-style is more compact for deeper nesting levels where
Setext-style would be awkward.

### Sentence case

Use sentence case for headings (capitalize only the first word and proper
nouns):

\`\`\`markdown
Development commands ← Correct Development Commands ← Incorrect
\`\`\`

_Rationale_: Sentence case is easier to read and more natural in technical
documentation.

### Underline length

The underline of a Setext-style heading should match the display width of the
heading text, accounting for East Asian wide characters.

#### East Asian character width

East Asian wide characters (CJK characters) are counted as two columns when
calculating the display width.

## Emphasis

### Asterisks for emphasis

Use asterisks (\`*\`) for emphasis by default:

\`\`\`markdown
This is _emphasized_ text. This is **strongly emphasized** text.
\`\`\`

### Underscores when content contains asterisks

When the emphasized content contains asterisk characters, use underscores to
avoid escaping:

\`\`\`markdown
The file _*.txt_ matches all text files. The pattern ****/*.md** matches
recursively.
\`\`\`

_Rationale_: This produces cleaner source text by avoiding backslash escapes.

### Escape all underscores in regular text

Underscores in regular text are always escaped, even in the middle of words:

\`\`\`markdown
Use the CONFIG\\_FILE\\_NAME constant.
\`\`\`

_Rationale_: While CommonMark doesn't treat intraword underscores as emphasis
delimiters, escaping ensures consistent rendering across all Markdown parsers.

## Lists

### Unordered list markers

Use \`-\` (space, hyphen, two spaces) for unordered list items:

\`\`\`markdown
- First item
- Second item
- Third item
\`\`\`

_Rationale_: The leading space creates visual indentation from the left margin.
The two trailing spaces align the text with a 4-space tab stop, making
continuation lines easy to align.

### Nested lists

Indent nested items by 4 spaces:

\`\`\`markdown
- Parent item
  - Child item
  - Another child
- Another parent
\`\`\`

### Ordered list markers

Use \`.\` for odd nesting levels and \`)\` for even nesting levels:

\`\`\`markdown
1. First item
2. Second item
   1. Nested first
   2. Nested second
3. Third item
\`\`\`

_Rationale_: Alternating markers make the nesting level visually apparent.

### Fixed marker width

Ordered list markers maintain a fixed 4-character width. When numbers grow
longer, trailing spaces are reduced (minimum 1 space):

\`\`\`markdown
1. First item
2. Second item ...
3. Ninth item
4. Tenth item
\`\`\`

_Rationale_: Consistent marker width keeps continuation lines aligned at the
same column regardless of item count.

### Continuation lines

Align continuation lines with the start of the item text:

\`\`\`markdown
- This is a list item with text that continues on the next line with proper
  alignment.
\`\`\`

### Task lists

Task list items use checkboxes (\`[ ]\` for unchecked, \`[x]\` for checked) after
the list marker:

\`\`\`markdown
- [ ] Unchecked task
- [x] Completed task
\`\`\`

_Rationale_: Task lists follow the same spacing rules as regular unordered
lists, keeping the document consistent.

## Code

### Fenced code blocks with tildes
`;
