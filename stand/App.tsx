import React from 'react';
import { StyleSheet, Text, SafeAreaView } from 'react-native';

import { Recipe, fetchRecipe } from './src/recipe/types';
import RecipeView from './src/recipe/view';

interface AppState {
  recipe: null | Recipe;
  loading: boolean;
  error: null | Error;
}

export default class App extends React.Component<{}, AppState> {
  state = { loading: true, error: null, recipe: null };
  componentDidMount() {
    (async () => {
      try {
        const recipe = await fetchRecipe();
        this.setState({
          loading: false,
          recipe,
        });
      } catch (err) {
        this.setState({ loading: false, error: err });
      }
    })();
  }
  render() {
    const state = this.state;
    const loading = state.loading;
    const recipe = state.recipe;
    return (
      <SafeAreaView style={styles.container}>
        {loading && <Text>Wczytuję...</Text>}
        {recipe && <RecipeView recipe={recipe} />}
      </SafeAreaView>
    );
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
  },
});
