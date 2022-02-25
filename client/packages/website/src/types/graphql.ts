import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  /**
   * ISO 8601 combined date and time without timezone.
   *
   * # Examples
   *
   * * `2015-07-01T08:59:60.123`,
   */
  NaiveDateTime: any;
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as Strings
   * within GraphQL. UUIDs are used to assign unique identifiers to entities without requiring a central
   * allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: any;
};

export type Mutation = {
  __typename?: 'Mutation';
  createPost: Post;
  createTag: Tag;
  deletePost: Post;
  deleteTag: Tag;
  updatePost: Post;
  updateTag: Tag;
};


export type MutationCreatePostArgs = {
  post: PostCreate;
};


export type MutationCreateTagArgs = {
  tag: TagCreate;
};


export type MutationDeletePostArgs = {
  id: Scalars['UUID'];
};


export type MutationDeleteTagArgs = {
  id: Scalars['UUID'];
};


export type MutationUpdatePostArgs = {
  id: Scalars['UUID'];
  post: PostUpdate;
};


export type MutationUpdateTagArgs = {
  id: Scalars['UUID'];
  tag: TagUpdate;
};

/** Information about pagination in a connection */
export type PageInfo = {
  __typename?: 'PageInfo';
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']>;
};

export type PaginationParams = {
  after?: InputMaybe<Scalars['String']>;
  before?: InputMaybe<Scalars['String']>;
  first?: InputMaybe<Scalars['Int']>;
  last?: InputMaybe<Scalars['Int']>;
};

export type Post = {
  __typename?: 'Post';
  createdAt: Scalars['NaiveDateTime'];
  id: Scalars['UUID'];
  readingTime: Scalars['Int'];
  slug: Scalars['String'];
  tags: Array<Tag>;
  title: Scalars['String'];
  updatedAt?: Maybe<Scalars['NaiveDateTime']>;
  visible: Scalars['Boolean'];
};

export type PostConnection = {
  __typename?: 'PostConnection';
  /** A list of edges. */
  edges?: Maybe<Array<Maybe<PostEdge>>>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
  totalCount: Scalars['Int'];
};

export type PostCreate = {
  readingTime: Scalars['Int'];
  slug: Scalars['String'];
  tagIds: Array<Scalars['UUID']>;
  title: Scalars['String'];
  visible: Scalars['Boolean'];
};

/** An edge in a connection. */
export type PostEdge = {
  __typename?: 'PostEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: Post;
};

export type PostUpdate = {
  readingTime?: InputMaybe<Scalars['Int']>;
  slug?: InputMaybe<Scalars['String']>;
  tagIds?: InputMaybe<Array<Scalars['UUID']>>;
  title?: InputMaybe<Scalars['String']>;
  visible?: InputMaybe<Scalars['Boolean']>;
};

export type Query = {
  __typename?: 'Query';
  post: Post;
  posts: PostConnection;
  tag: Tag;
  tags: TagConnection;
};


export type QueryPostArgs = {
  id: Scalars['UUID'];
};


export type QueryPostsArgs = {
  params: PaginationParams;
};


export type QueryTagArgs = {
  id: Scalars['UUID'];
};


export type QueryTagsArgs = {
  params: PaginationParams;
};

export type Tag = {
  __typename?: 'Tag';
  createdAt: Scalars['NaiveDateTime'];
  icon?: Maybe<Scalars['String']>;
  id: Scalars['UUID'];
  name: Scalars['String'];
  updatedAt?: Maybe<Scalars['NaiveDateTime']>;
};

export type TagConnection = {
  __typename?: 'TagConnection';
  /** A list of edges. */
  edges?: Maybe<Array<Maybe<TagEdge>>>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
  totalCount: Scalars['Int'];
};

export type TagCreate = {
  icon?: InputMaybe<Scalars['String']>;
  name: Scalars['String'];
};

/** An edge in a connection. */
export type TagEdge = {
  __typename?: 'TagEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String'];
  /** The item at the end of the edge */
  node: Tag;
};

export type TagUpdate = {
  icon?: InputMaybe<Scalars['String']>;
  name?: InputMaybe<Scalars['String']>;
};

export type TagsQueryVariables = Exact<{ [key: string]: never; }>;


export type TagsQuery = { __typename?: 'Query', tags: { __typename?: 'TagConnection', edges?: Array<{ __typename?: 'TagEdge', cursor: string, node: { __typename?: 'Tag', id: any, name: string, icon?: string | null } } | null> | null } };

export type PostsQueryVariables = Exact<{
  after?: InputMaybe<Scalars['String']>;
  before?: InputMaybe<Scalars['String']>;
  first?: InputMaybe<Scalars['Int']>;
  last?: InputMaybe<Scalars['Int']>;
}>;


export type PostsQuery = { __typename?: 'Query', posts: { __typename?: 'PostConnection', totalCount: number, edges?: Array<{ __typename?: 'PostEdge', node: { __typename?: 'Post', id: any, title: string, slug: string, readingTime: number, createdAt: any, updatedAt?: any | null, tags: Array<{ __typename?: 'Tag', id: any, name: string, icon?: string | null }> } } | null> | null, pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null } } };


export const TagsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"tags"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tags"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"params"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"first"},"value":{"kind":"IntValue","value":"4"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"cursor"}},{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"icon"}}]}}]}}]}}]}}]} as unknown as DocumentNode<TagsQuery, TagsQueryVariables>;
export const PostsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"posts"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"after"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"before"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"first"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"last"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"posts"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"params"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"after"},"value":{"kind":"Variable","name":{"kind":"Name","value":"after"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"before"},"value":{"kind":"Variable","name":{"kind":"Name","value":"before"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"first"},"value":{"kind":"Variable","name":{"kind":"Name","value":"first"}}},{"kind":"ObjectField","name":{"kind":"Name","value":"last"},"value":{"kind":"Variable","name":{"kind":"Name","value":"last"}}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"totalCount"}},{"kind":"Field","name":{"kind":"Name","value":"edges"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"node"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"slug"}},{"kind":"Field","name":{"kind":"Name","value":"readingTime"}},{"kind":"Field","name":{"kind":"Name","value":"createdAt"}},{"kind":"Field","name":{"kind":"Name","value":"updatedAt"}},{"kind":"Field","name":{"kind":"Name","value":"tags"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"name"}},{"kind":"Field","name":{"kind":"Name","value":"icon"}}]}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"pageInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"hasPreviousPage"}},{"kind":"Field","name":{"kind":"Name","value":"hasNextPage"}},{"kind":"Field","name":{"kind":"Name","value":"startCursor"}},{"kind":"Field","name":{"kind":"Name","value":"endCursor"}}]}}]}}]}}]} as unknown as DocumentNode<PostsQuery, PostsQueryVariables>;