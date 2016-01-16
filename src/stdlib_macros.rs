macro_rules! vec {
	( $ elem : expr ; $ n : expr ) => ( $ crate:: vec:: from_elem ( $ elem , $ n )
	) ; ( $ ( $ x : expr ) , * ) => (
	< [ _ ] > :: into_vec ( $ crate:: boxed:: Box:: new ( [ $ ( $ x ) , * ] ) ) )
	; ( $ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] );
}