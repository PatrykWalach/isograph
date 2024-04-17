import type {EagerReaderArtifact, ReaderAst, RefetchQueryNormalizationArtifact} from '@isograph/react';
import { Query__meNameSuccessor__param } from './param_type';
import { Query__meNameSuccessor__outputType } from './output_type';
import { meNameField as resolver } from '../../../meNameSuccessor.ts';

const readerAst: ReaderAst<Query__meNameSuccessor__param> = [
  {
    kind: "Linked",
    fieldName: "me",
    alias: null,
    arguments: null,
    selections: [
      {
        kind: "Scalar",
        fieldName: "name",
        alias: null,
        arguments: null,
      },
      {
        kind: "Linked",
        fieldName: "successor",
        alias: null,
        arguments: null,
        selections: [
          {
            kind: "Linked",
            fieldName: "successor",
            alias: null,
            arguments: null,
            selections: [
              {
                kind: "Scalar",
                fieldName: "name",
                alias: null,
                arguments: null,
              },
            ],
          },
        ],
      },
    ],
  },
];

const artifact: EagerReaderArtifact<
  Query__meNameSuccessor__param,
  Query__meNameSuccessor__outputType
> = {
  kind: "EagerReaderArtifact",
  resolver,
  readerAst,
};

export default artifact;
