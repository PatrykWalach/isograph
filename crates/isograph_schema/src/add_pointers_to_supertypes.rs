use std::vec;

use common_lang_types::{SelectableFieldName, Span, UnvalidatedTypeName, WithLocation, WithSpan};
use graphql_lang_types::{GraphQLNamedTypeAnnotation, GraphQLTypeAnnotation};
use intern::string_key::Intern;
use isograph_lang_types::{ScalarFieldSelection, Selection, ServerFieldSelection};

use crate::{
    FieldType, ProcessTypeDefinitionError, ProcessTypeDefinitionResult, SchemaObject,
    SchemaServerField, SchemaServerFieldInlineFragmentVariant, SchemaServerFieldVariant,
    TypeRefinementMap, UnvalidatedSchema, ValidatedIsographSelectionVariant,
    ValidatedScalarFieldAssociatedData,
};
use common_lang_types::Location;
impl UnvalidatedSchema {
    /// For each supertype (e.g. Node), add the pointers for each subtype (e.g. User implements Node)
    /// to supertype (e.g. creating Node.asUser).
    pub fn add_pointers_to_supertypes(
        &mut self,
        subtype_to_supertype_map: &TypeRefinementMap,
    ) -> ProcessTypeDefinitionResult<()> {
        for (subtype_id, supertype_ids) in subtype_to_supertype_map {
            let subtype: &SchemaObject = self.server_field_data.object(*subtype_id);

            if let Some(id_field) = subtype.id_field {
                if let Some(concrete_type) = subtype.concrete_type {
                    let field_name: SelectableFieldName =
                        format!("as{}", subtype.name).intern().into();

                    let next_server_field_id = self.server_fields.len().into();

                    let associated_data: GraphQLTypeAnnotation<UnvalidatedTypeName> =
                        GraphQLTypeAnnotation::Named(GraphQLNamedTypeAnnotation(WithSpan {
                            item: subtype.name.into(),
                            span: Span::todo_generated(),
                        }));

                    let id_selection = WithSpan::new(
                        Selection::ServerField(ServerFieldSelection::ScalarField(
                            ScalarFieldSelection {
                                arguments: vec![],
                                associated_data: ValidatedScalarFieldAssociatedData {
                                    location: FieldType::ServerField(id_field.into()),
                                    selection_variant: ValidatedIsographSelectionVariant::Regular,
                                },
                                directives: vec![],
                                name: WithLocation::new(
                                    "id".intern().into(),
                                    Location::generated(),
                                ),
                                reader_alias: None,
                                unwraps: vec![],
                            },
                        )),
                        Span::todo_generated(),
                    );
                    let typename_selection = WithSpan::new(
                        Selection::ServerField(ServerFieldSelection::ScalarField(
                            ScalarFieldSelection {
                                arguments: vec![],
                                associated_data: ValidatedScalarFieldAssociatedData {
                                    location: FieldType::ServerField(
                                        *subtype
                                            .encountered_fields
                                            .get(&"__typename".intern().into())
                                            .expect("Expected __typename to exist")
                                            .as_server_field()
                                            .expect("Expected __typename to be server field"),
                                    ),
                                    selection_variant: ValidatedIsographSelectionVariant::Regular,
                                },
                                directives: vec![],
                                name: WithLocation::new(
                                    "__typename".intern().into(),
                                    Location::generated(),
                                ),
                                reader_alias: None,
                                unwraps: vec![],
                            },
                        )),
                        Span::todo_generated(),
                    );

                    let condition_selection_set = vec![id_selection, typename_selection];

                    let server_field = SchemaServerField {
                        description: Some(
                            format!("A client poiter for the {} type.", subtype.name)
                                .intern()
                                .into(),
                        ),
                        id: next_server_field_id,
                        name: WithLocation::new(field_name, Location::generated()),
                        parent_type_id: subtype.id,
                        arguments: vec![],
                        associated_data,
                        is_discriminator: false,
                        variant: SchemaServerFieldVariant::InlineFragment(
                            SchemaServerFieldInlineFragmentVariant {
                                server_field_id: next_server_field_id,
                                concrete_type,
                                condition_selection_set,
                            },
                        ),
                    };

                    self.server_fields.push(server_field);

                    for supertype_id in supertype_ids {
                        let supertype = self.server_field_data.object_mut(*supertype_id);

                        if supertype
                            .encountered_fields
                            .insert(field_name, FieldType::ServerField(next_server_field_id))
                            .is_some()
                        {
                            return Err(WithLocation::new(
                                ProcessTypeDefinitionError::FieldExistsOnType {
                                    field_name,
                                    parent_type: supertype.name,
                                },
                                Location::generated(),
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
