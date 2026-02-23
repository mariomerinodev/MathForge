export function toLatex(rustOutput: string): string {
    if (!rustOutput || rustOutput === "0" || rustOutput === "Error") return rustOutput;

    let tex = rustOutput;

    // 1. Limpieza inicial: Quitar paréntesis externos innecesarios
    while (tex.startsWith('(') && tex.endsWith(')') && areParenthesesBalanced(tex.slice(1, -1))) {
        tex = tex.slice(1, -1);
    }

    // 2. Raíces Cuadradas: Convertir (N ^ 1/2) en \sqrt{N}
    tex = tex.replace(/\((.*?) \^ 1\/2\)/g, '\\sqrt{$1}');

    // 3. Fracciones: Convertir A/B en \frac{A}{B}
    tex = tex.replace(/(\d+|[a-z])\/(\d+|[a-z])/g, '\\frac{$1}{$2}');

    // 4. Multiplicación: El motor usa " * ". En álgebra visual, solemos omitirlo
    tex = tex.replace(/ \* /g, '');

    // 5. Limpieza final de paréntesis redundantes
    tex = tex.replace(/\\sqrt{\((.*?)\)}/g, '\\sqrt{$1}');
    tex = tex.replace(/\\frac{\((.*?)\)}{\((.*?)\)}/g, '\\frac{$1}{$2}');

    return tex;
}

// Función auxiliar para saber si podemos quitar paréntesis externos con seguridad
function areParenthesesBalanced(str: string): boolean {
    let depth = 0;
    for (const char of str) {
        if (char === '(') depth++;
        else if (char === ')') depth--;
        if (depth < 0) return false;
    }
    return depth === 0;
}