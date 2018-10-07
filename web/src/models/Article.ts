export default interface Article extends ListViewArticle {
  text: string;
}


export function newArticle(): Article {
  return {
    user_id: 0,
    id: '',
    title: '',
    text: '',
    tags: [],
  };
}

export interface ListViewArticle {
  id: string;
  user_id: number;
  title: string;
  tags: string[];
}

export function setTags(article: ListViewArticle, tags: string) {
  article.tags = tags.split(',').map((t) => t.trim());
}

export function getTags(article: ListViewArticle): string {
  return article.tags.join(', ');
}

