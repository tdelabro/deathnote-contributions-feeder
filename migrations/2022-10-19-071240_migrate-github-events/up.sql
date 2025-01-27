UPDATE
	events
SET
	payload = jsonb_build_object('Contributor',
			jsonb_build_object('GithubAccountAssociated',
				jsonb_build_object(
                    'contributor_account_address', payload->'Contributor'->'GithubAccountAssociated'->'contributor_account',
                    'github_identifier', payload->'Contributor'->'GithubAccountAssociated'->'github_identifier'
				)
			)
		)
WHERE
	payload -> 'Contributor' ? 'GithubAccountAssociated';
