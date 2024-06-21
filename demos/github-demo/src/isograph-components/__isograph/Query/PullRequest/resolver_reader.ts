import type {ComponentReaderArtifact, ExtractSecondParam, ReaderAst, RefetchQueryNormalizationArtifact} from '@isograph/react';
import { Query__PullRequest__param } from './param_type';
import { PullRequest as resolver } from '../../../PullRequestRoute';
import Query__Header__resolver_reader from '../../Query/Header/resolver_reader';
import Query__PullRequestDetail__resolver_reader from '../../Query/PullRequestDetail/resolver_reader';

const readerAst: ReaderAst<Query__PullRequest__param> = [
  {
    kind: "Resolver",
    alias: "Header",
    arguments: null,
    readerArtifact: Query__Header__resolver_reader,
    usedRefetchQueries: [],
  },
  {
    kind: "Resolver",
    alias: "PullRequestDetail",
    arguments: null,
    readerArtifact: Query__PullRequestDetail__resolver_reader,
    usedRefetchQueries: [],
  },
];

const artifact: ComponentReaderArtifact<
  Query__PullRequest__param,
  ExtractSecondParam<typeof resolver>
> = {
  kind: "ComponentReaderArtifact",
  componentName: "Query.PullRequest",
  resolver,
  readerAst,
};

export default artifact;
