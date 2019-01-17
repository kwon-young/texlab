package texlab.completion.bibtex

import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.Test
import texlab.WorkspaceBuilder

class BibtexKernelCommandProviderTests {
    @Test
    fun `it should provide items when inside of a command`() {
        WorkspaceBuilder()
                .document("foo.bib", "@article{foo, bar=\n\\}")
                .completion("foo.bib", 1, 1)
                .let { BibtexKernelCommandProvider.complete(it) }
                .also { assertFalse(it.isEmpty()) }
    }

    @Test
    fun `it should not provide items when inside of text`() {
        WorkspaceBuilder()
                .document("foo.bib", "@article{foo, bar=\n}")
                .completion("foo.bib", 1, 0)
                .let { BibtexKernelCommandProvider.complete(it) }
                .also { assertTrue(it.isEmpty()) }
    }

    @Test
    fun `it should not provide items in LaTeX documents`() {
        WorkspaceBuilder()
                .document("foo.tex", "\\")
                .completion("foo.tex", 0, 1)
                .let { BibtexKernelCommandProvider.complete(it) }
                .also { assertTrue(it.isEmpty()) }
    }
}