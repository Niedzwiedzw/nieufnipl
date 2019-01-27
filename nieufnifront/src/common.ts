import { API_BASE } from '@/config';

export interface Article {
    id: string;
    title: string;
    date: string;
    author: string;
    markdown_text: string;
    rendered_text: string;
}

function capitalizeFirstLetter(word: string) {
    return word.charAt(0).toUpperCase() + word.slice(1);
}

export class Metatag {
    constructor(public property: string, public content: string) {}
    public get asHtml(): string { return `<meta property="og:${this.property}" content="${this.content}">`; }
    public inject() { document.head.innerHTML += this.asHtml; }
}

export function humanTitle(title: string): string {
    return title.split('-').map(capitalizeFirstLetter).join(' ');
}

export function emptyArticle(): Article {
    return {
        id: '',
        title: '',
        date: '',
        author: '',
        markdown_text: '',
        rendered_text: '',
    };
}

export async function getArticle(id: string): Promise<Article> {
    const response = await fetch(`${API_BASE}/artykuly/${id}`);
    return await response.json();
}

export async function allArticles(): Promise<Article[]> {
    const response = await fetch(`${API_BASE}/artykuly`);
    return await response.json();
}
