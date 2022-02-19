declare namespace Model {
  /* ----------------------------------------------------------------
  Blog
  ---------------------------------------------------------------- */
  export interface PostTag {
    id: string;
    name: string;
    icon?: string;
  }

  export interface PostTagR extends PostTag {
    active?: boolean;
  }

  export interface Post {
    id: string;
    slug: string;
    title: string;
    createdAt: string;
    updatedAt?: string;
    readingTime: number;
    tags: PostTag[];
    body: string;
  }

  export interface PostR extends Omit<Post, 'createdAt' | 'updatedAt'> {
    id: string;
    slug: string;
    title: string;
    createdAt: Date;
    updatedAt?: Date;
    readingTime: number;
    tags: PostTag[];
    body: string;
  }

  export type PostListItem = Omit<Post, 'body'>;

  export type PostListItemR = Omit<PostR, 'body'>;
}
