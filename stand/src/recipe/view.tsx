import React from 'react';
import { Text, View, ScrollView, RefreshControl } from 'react-native';
import styled from 'styled-components';

import { Recipe } from './types';

interface RecipeProps {
  recipe: Recipe;
  refreshing: boolean;
  refresh: () => void;
}

const RecipeView = ({ recipe, refresh, refreshing }: RecipeProps) => {
  return (
    <RecipeContainer refreshControl={<RefreshControl refreshing={refreshing} onRefresh={refresh} />}>
      <RecipeName> {recipe.name} </RecipeName>
      {recipe.ingredients.map(({ name }, index) => (
        <Ingredient key={index}>{name}</Ingredient>
      ))}
      {recipe.preparation.map(({ description }, index) => (
        <Ingredient key={index}>{description}</Ingredient>
      ))}
    </RecipeContainer>
  );
};

export default RecipeView;

const RecipeContainer = styled(ScrollView)`
  margin: 5px 10px;
`;

const RecipeName = styled(Text)`
  font-size: 32;
`;

const Ingredient = styled(Text)`
  margin: 5px 0;
`;
