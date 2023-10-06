import {type ITag} from './Tags';

export interface IQuiz {
    qid:   string;
    title: string;
    tags:  ITag[];
    description? : string;
}