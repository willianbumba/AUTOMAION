package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p>Profile ADM POLARIS : POLARIS
Profile ADM SIRIUS : SIRIUS
Profile PORTAL POLARIS : PORTAL CONTRIBUINTE POLARIS</p>
     */
    public static Object URL
     
    /**
     * <p>Profile ADM POLARIS : ID Utilizador
Profile PORTAL POLARIS : ID Utilizador</p>
     */
    public static Object utilizador
     
    /**
     * <p>Profile ADM POLARIS : Palavra-passe
Profile PORTAL POLARIS : Palavra-passe</p>
     */
    public static Object senha
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += TestCaseMain.getParsedValues(RunConfiguration.getOverridingParameters())
    
            URL = selectedVariables['URL']
            utilizador = selectedVariables['utilizador']
            senha = selectedVariables['senha']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
