import { API_BASE } from '@/config';

export interface NewArticleData {
    username: string;
    password: string;
    articleTitle: string;
}

export interface HttpHeaders {
    [key: string]: string;
}


export interface Article {
    id: string;
    title: string;
    date: string;
    author: string;
    markdown_text: string;
    rendered_text: string;
}

export interface ArticleCreated {
    id: string;
    title: string;
    date: string;
    authors_id: number;
    markdown_text: string;
    rendered_text: string;

    username: string;
    password: string;
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
    if (!response.ok) { throw new Error(response.statusText); }
    return await response.json();
}

export async function allArticles(): Promise<Article[]> {
    const response = await fetch(`${API_BASE}/artykuly`);
    return await response.json();
}


export async function post(url: string, data: object, username?: string, password?: string) {
    const headers: HttpHeaders = {
        'Content-Type': 'application/json',
    };

    if (username && password) {
        headers['not-trustworthy-username'] = username;
        headers['not-trustworthy-password'] = password;
    }
    const jsonData = JSON.stringify(data);
    console.log('sending', data, jsonData);

    fetch(url,
        {
            method: 'POST',
            headers,
            body: jsonData,
        }).then((response) => response.ok)
        .catch((e) => console.error(e));
}

export async function sendArticle(
    title: string,
    markdownText: string,
    username: string,
    password: string,
): Promise<boolean> {
    const article: ArticleCreated = {
        title,
        username,
        password,
        markdown_text: markdownText,
        authors_id: 0,  // empty
        id: '',  // empty
        date: '',  // empty
        rendered_text: '',  // empty
    };

    try {
        post('/api/artykuly/', article, username, password);
        return true;
    } catch (e) {
        // tslint:disable-next-line:no-console
        console.error(e);
        return false;
    }
}
