import { createSignal, onMount } from 'solid-js';
import core, { add } from '../chess-core/core';

const App = () => {
	const [result, setResult] = createSignal<number>();

	onMount(async () => {
		await core();
		setResult(add(1, 2));
	});

	return <div>{result()}</div>;
};

export default App;
