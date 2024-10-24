import type { IsographEntrypoint, ReaderAst, FragmentReference, NormalizationAst, RefetchQueryNormalizationArtifact } from '@isograph/react';
const queryText = 'query User__refetch ($id: ID!) {\
  node____id___v_id: node(id: $id) {\
    ... on User {\
      __typename,\
      id,\
      avatarUrl,\
      login,\
      name,\
      repositories____first___l_10____after___l_null: repositories(first: 10, after: null) {\
        edges {\
          node {\
            id,\
            description,\
            forkCount,\
            name,\
            nameWithOwner,\
            owner {\
              __typename,\
              id,\
              login,\
            },\
            pullRequests {\
              totalCount,\
            },\
            stargazerCount,\
            watchers {\
              totalCount,\
            },\
          },\
        },\
        pageInfo {\
          endCursor,\
          hasNextPage,\
        },\
      },\
    },\
  },\
}';

const normalizationAst: NormalizationAst = [
  {
    kind: "Linked",
    fieldName: "node",
    arguments: [
      [
        "id",
        { kind: "Variable", name: "id" },
      ],
    ],
    concreteType: null,
    selections: [
      {
        kind: "InlineFragment",
        type: "User",
        selections: [
          {
            kind: "Scalar",
            fieldName: "__typename",
            arguments: null,
          },
          {
            kind: "Scalar",
            fieldName: "id",
            arguments: null,
          },
          {
            kind: "Scalar",
            fieldName: "avatarUrl",
            arguments: null,
          },
          {
            kind: "Scalar",
            fieldName: "login",
            arguments: null,
          },
          {
            kind: "Scalar",
            fieldName: "name",
            arguments: null,
          },
          {
            kind: "Linked",
            fieldName: "repositories",
            arguments: [
              [
                "first",
                { kind: "Literal", value: 10 },
              ],

              [
                "after",
                { kind: "Literal", value: null },
              ],
            ],
            concreteType: "RepositoryConnection",
            selections: [
              {
                kind: "Linked",
                fieldName: "edges",
                arguments: null,
                concreteType: "RepositoryEdge",
                selections: [
                  {
                    kind: "Linked",
                    fieldName: "node",
                    arguments: null,
                    concreteType: "Repository",
                    selections: [
                      {
                        kind: "Scalar",
                        fieldName: "id",
                        arguments: null,
                      },
                      {
                        kind: "Scalar",
                        fieldName: "description",
                        arguments: null,
                      },
                      {
                        kind: "Scalar",
                        fieldName: "forkCount",
                        arguments: null,
                      },
                      {
                        kind: "Scalar",
                        fieldName: "name",
                        arguments: null,
                      },
                      {
                        kind: "Scalar",
                        fieldName: "nameWithOwner",
                        arguments: null,
                      },
                      {
                        kind: "Linked",
                        fieldName: "owner",
                        arguments: null,
                        concreteType: null,
                        selections: [
                          {
                            kind: "Scalar",
                            fieldName: "__typename",
                            arguments: null,
                          },
                          {
                            kind: "Scalar",
                            fieldName: "id",
                            arguments: null,
                          },
                          {
                            kind: "Scalar",
                            fieldName: "login",
                            arguments: null,
                          },
                        ],
                      },
                      {
                        kind: "Linked",
                        fieldName: "pullRequests",
                        arguments: null,
                        concreteType: "PullRequestConnection",
                        selections: [
                          {
                            kind: "Scalar",
                            fieldName: "totalCount",
                            arguments: null,
                          },
                        ],
                      },
                      {
                        kind: "Scalar",
                        fieldName: "stargazerCount",
                        arguments: null,
                      },
                      {
                        kind: "Linked",
                        fieldName: "watchers",
                        arguments: null,
                        concreteType: "UserConnection",
                        selections: [
                          {
                            kind: "Scalar",
                            fieldName: "totalCount",
                            arguments: null,
                          },
                        ],
                      },
                    ],
                  },
                ],
              },
              {
                kind: "Linked",
                fieldName: "pageInfo",
                arguments: null,
                concreteType: "PageInfo",
                selections: [
                  {
                    kind: "Scalar",
                    fieldName: "endCursor",
                    arguments: null,
                  },
                  {
                    kind: "Scalar",
                    fieldName: "hasNextPage",
                    arguments: null,
                  },
                ],
              },
            ],
          },
        ],
      },
    ],
  },
];
const artifact: RefetchQueryNormalizationArtifact = {
  kind: "RefetchQuery",
  queryText,
  concreteType: "Query",
  queryType: "Query",
  normalizationAst,
};

export default artifact;
