export async function get_langs(): Promise<JSON[]> {
    const url: String = "http://localhost:8080/langs";
    // const data: String = "Hello";

    // This is using text so it might be horrible yay!!!
    const result: JSON[] = await fetch(url, {
        method: "GET",
        headers: {"Accept": "application/json"},
    }).then((response) => response.json());
    const sorted: JSON[] = result.sort((a, b) => {
        if (a.display < b.display)
            return -1;
        if (a.display > b.display)
            return 1;
        return 0;
    });
    return sorted;
}

function random_number(min, max) {
    return Math.floor(Math.random() * (max - min) + min);
}

// Makes a random color
export function random_color(): String {
    const from = 30;
    const to = 255;
    const r1: Integer = random_number(from, to);
    const r2: Integer = random_number(from, to);
    const r3: Integer = random_number(from, to);

    const h1: String = r1.toString(16);
    const h2: String = r2.toString(16);
    const h3: String = r3.toString(16);

    return "#" + h1 + h2 + h3
}

export function pastel_color(): String { 
    return "hsl(" + 360 * Math.random() + ',' +
                (25 + 70 * Math. random()) + '%,' + 
                (85 + 10 * Math.random()) + '%)'
}

export const all_langs: Promise<JSON[]> = get_langs();
export let langs: Promise<JSON[]> = all_langs;

export function refresh_langs(new_langs: JSON[]) {
    langs = new Promise((resolve, reject) => {
        setTimeout(() => { resolve(new_langs); }, 100);
    });
}
