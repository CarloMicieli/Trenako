package io.github.carlomicieli.trenako

import io.github.carlomicieli.trenako.model.ProblemDetail

data class ProblemDetailException(val problemDetail: ProblemDetail) : RuntimeException(problemDetail.title)
