use oracle::Connection;
use oracle::sql_type::RefCursor;
use crate::domain::models::model::LlmInfo;

pub struct ModelRepository;

impl ModelRepository {
    pub fn fetch_all_models(conn: &Connection) -> Result<Vec<LlmInfo>, oracle::Error> {
        // 1. Prepara o bloco PL/SQL chamando a procedure com o parâmetro :1
        let mut stmt = conn
            .statement("BEGIN llm_metadata_api.get_all(:1); END;")
            .build()?;
        
        // 2. Executa passando um ponteiro nulo para o Oracle alocar o RefCursor de saída
        stmt.execute(&[&None::<RefCursor>])?;
        
        // 3. CORRIGIDO: ref_cursor marcado como mutável (mut)
        let mut ref_cursor: RefCursor = stmt.bind_value(1)?;
        
        // 4. O método .query() consome o ref_cursor mutável para ler os dados do banco
        let mut rows = ref_cursor.query()?;
        let mut models = Vec::new();

        // 5. Itera sobre as linhas de maneira limpa e performática
        while let Some(row_result) = rows.next() {
            let row = row_result?;
            models.push(LlmInfo {
                id: row.get(0)?,          
                model_name: row.get(1)?,  
                version: row.get(2)?,     
                provider: row.get(3)?,    
            });
        }

        Ok(models)
    }
}
