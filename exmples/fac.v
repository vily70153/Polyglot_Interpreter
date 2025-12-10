функція Factorial(ціле n) {
        якщо (n < 2) {
            повернути 1
        } інакше {
            повернути n * Factorial(n - 1)
        }
    }
    
змінна result = Factorial(5)
друк("Факторіал 5 дорівнює:", result)
