import { createSignal, onMount } from 'solid-js';
import { ChessBoard } from '../chess-core';

const App = () => {

	return <div>{ChessBoard.new().print()}</div>;
};

export default App;
