import type {MutationReaderArtifact, RefetchQueryNormalizationArtifact, ReaderAst} from '@isograph/react';
import { Pet__set_best_friend__param } from './param_type';
const includeReadOutData = (variables: any, readOutData: any) => {
  variables.id = readOutData.id;
  return variables;
};

import { makeNetworkRequest, type IsographEnvironment } from '@isograph/react';
const resolver = (
  environment: IsographEnvironment,
  artifact: RefetchQueryNormalizationArtifact,
  readOutData: any,
  filteredVariables: any
) => (mutationParams: any) => {
  const variables = includeReadOutData({...filteredVariables, ...mutationParams}, readOutData);
  makeNetworkRequest(environment, artifact, variables);
};


const readerAst: ReaderAst<Pet__set_best_friend__param> = [
  {
    kind: "Scalar",
    fieldName: "id",
    alias: null,
    arguments: null,
  },
];

const artifact: MutationReaderArtifact<
  Pet__set_best_friend__param
> = {
  kind: "MutationReaderArtifact",
  resolver,
  readerAst,
};

export default artifact;
