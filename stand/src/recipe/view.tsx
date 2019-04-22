import React from 'react';
import { Text, View, ScrollView, RefreshControl } from 'react-native';
import styled from 'styled-components';

import { Recipe } from './types';

interface RecipeProps {
  recipe: Recipe | null;
  refreshing: boolean;
  refresh: () => void;
}

const RecipeDetails = ({ recipe }: { recipe: Recipe }) => {
  return (
    <>
      <RecipeName>{recipe.name}</RecipeName>
      <Header>Składniki</Header>
      <StepsList>
        {recipe.ingredients.map(({ name }, index) => (
          <Step key={index}>{name}</Step>
        ))}
      </StepsList>
      <Header>Przygotowanie</Header>
      <StepsList>
        {recipe.preparation.map(({ description }, index) => (
          <Step key={index}>{description}</Step>
        ))}
      </StepsList>
    </>
  );
};

const RecipeView = ({ recipe, refresh, refreshing }: RecipeProps) => {
  return (
    <RecipeContainer refreshControl={<RefreshControl refreshing={refreshing} onRefresh={refresh} />}>
      {recipe && <RecipeDetails recipe={recipe} />}
    </RecipeContainer>
  );
};

export default RecipeView;

const RecipeContainer = styled(ScrollView)`
  margin: 5px 10px;
`;

const Header = styled(Text)`
  font-size: 32;
  font-weight: 800;
  margin: 10px 0;
`;

const RecipeName = styled(Header)`
  font-weight: 900;
`;

const StepsList = styled(View)`
  margin: 5px 0;
`;

const Step = styled(Text)`
  margin: 10px 0;
  font-size: 18;
  line-height: ${18 * 1.5};
  font-weight: 500;
`;
