<script lang="ts">

import RawOfQuiz from "../lib/RawOfQuiz.svelte";

import { type IQuiz } from "../types/Quiz";

let testQuiz: IQuiz[] = [
	{
		title: "Test",
		qid: "0a",
		description:"Lorem ipsum dolor sit amet, consectetur adipiscing elit. In ullamcorper dui eu mi tincidunt convallis. Nam molestie enim non vestibulum luctus. Cras aliquet vulputate metus, id auctor risus aliquet eu. Vestibulum eu dolor tincidunt, viverra velit et, vehicula lacus. Curabitur faucibus rhoncus turpis sit amet tristique. Donec non vehicula diam. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Integer metus purus, imperdiet nec semper in, placerat ac metus. Etiam a mauris in velit tristique condimentum a in enim. Proin varius nisi sed maximus rutrum. Mauris ullamcorper tellus et sem condimentum viverra. In suscipit enim bibendum arcu commodo, non laoreet tortor aliquam. Duis justo dui, aliquet et ligula vulputate, volutpat varius lectus. Fusce posuere rutrum sodales.",
		tags: [
			{
				name: "test",
				color: "#14A76C",
			},
		],
	},
	{
		title: "Test",
		qid: "0a",
		tags: [
			{
				name: "voici-un-long-tag",
				color: "#14A76C",
			},
		],
	},
	{
		title: "Test",
		qid: "0a",
		tags: [
			{
				name: "voici-un-long-tag",
				color: "#14A76C",
			},
		],
	},
	{
		title: "Test",
		qid: "0a",
		tags: [
			{
				name: "voici-un-long-tag",
				color: "#14A76C",
			},
		],
	},
];
</script>

<div id="myQuiz">
    <RawOfQuiz title="Mes Créations" quizzes={testQuiz} />
</div>

<div id="othersQuiz">
    <RawOfQuiz title="Mes Favoris" quizzes={testQuiz} />
</div>
