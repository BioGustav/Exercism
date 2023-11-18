import kotlin.math.*

data class ComplexNumber(val real: Double = 0.0, val imag: Double = 0.0) {
    val abs = sqrt(real.pow(2) + imag.pow(2))

    operator fun plus(other: ComplexNumber) = ComplexNumber(real + other.real, imag + other.imag)
    operator fun minus(other: ComplexNumber) = ComplexNumber(real - other.real, imag - other.imag)
    operator fun times(other: ComplexNumber) = ComplexNumber(
        real * other.real - imag * other.imag, imag * other.real + real * other.imag
    )

    operator fun div(other: ComplexNumber) = ComplexNumber(
        (real * other.real + imag * other.imag) / (other.real.pow(2) + other.imag.pow(2)),
        (imag * other.real - real * other.imag) / (other.real.pow(2) + other.imag.pow(2))
    )

    fun conjugate() = copy(imag = -imag)
}

fun exponential(exponent: ComplexNumber) = ComplexNumber(
    exp(exponent.real) * cos(exponent.imag), exp(exponent.real) * sin(exponent.imag)
)
