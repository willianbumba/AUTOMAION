<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Encerrar SessoDeseja continuar na sess_36cff3</name>
   <tag></tag>
   <elementGuidId>91682cb3-69cf-4e12-a3f0-7c23c85f5f1d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>ef734577-9913-48eb-bd49-9fa5ef634047</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


			
			
				
					
						
							
							Encerrar Sessão
						
						
							
								
							
							
								Deseja continuar na sessão do Portal do Contribuinte?
								Continuar
								Sair
							

						

					
				
			

			
				
					

					
					
					
					

					

					
						
							

							

						
						
							19:22
						

						
							
								
									
										Toggle navigation
									
								

									
											
												
													DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
													
												
												
													NIF TESTE PROJECTO SIGT
													MUDAR DE CONTA
												
											
									
							
						

						
						
								Sair
						
						
					

				

				

					
						

							
								Portal do Contribuinte
							

							
							
								
									
										Toggle navigation
										
										
										
									
								
								
									
										Início
										
											Serviços
											
													Notificações e Comunicações 
													
														Cadastro de Contribuinte
														
															Consultar Cadastro 
															Alterar Cadastro 
															Criar Modelo 6 
															Emitir Comprovativo
															Consultar Modelo 6
															Emitir Certificado
														
													
												
                                                        Património
                                                        
                                                            
                                                                Consultar Património
                                                                
                                                                    Pré-Cadastro Electrónico
                                                                    
                                                                        Cadastrar Fracção
                                                                        Consultar / Cadastrar Património
                                                                    
                                                                
                                                            
                                                        
                                                    

													
														Liquidações
														
															Liquidar Imposto 
															Consultar Liquidações
														
													
													
														Pagamentos
														
															Histórico de Pagamentos
															
															Verificar Nº NL / RP
														
													

													
														Certidão de Não Devedor
														
															Solicitar
															Verificar
														
													

													
														Conta Corrente
														
															
																Extracto de Conta Corrente
															
																
																	Certidão de não Devedor
																
																i

														
													
													
														Declarações
														
																
																	II
																	
																		
																			Consultar

																			
																				Declaração Anual
																			

																		
																		
																			Entregar

																			
																				Declaração Anual
																			

																		
																	
																

															
																IRT
																
																	
																		IRT
																
															

															
																IEC
																
																	
																		Consultar

																		
																			Declaração de IEC
																		

																	
																	
																		Entregar

																		
																			Declaração de IEC
																		

																	
																
															
															
																IVA
																
																	
																		Consultar

																		
																			Declaração de IVA
																			Mapa de Fornecedores
																		

																	
																	
																		Entregar

																		
																			Declaração de IVA
																			Mapa de Fornecedores
																		

																	
																
															

















															
																Gestão de Facturação
																
																	
																		Submissão do SAF-T
																	
																		Facturas Manuais
																
															
															
																IVM
																
																	
																		Submissão do IVM
																
															

														
													
													
														Reembolsos
														
															IVA
														
													
													
														Produtores de software
														
															
																	Submissão do Modelo 8
																	Consultar Certificado
															
														
													

													
														Verificar Certificado
														
															Residência Fiscal
														
													

													
														Gráficas e Tipografias
														
															
																	Solicitação de Licenciamento
																	Consultar Factura
															
														
													

													
															Emissão de Facturas
													

													
														Gestão do Utilizador
														
															Alterar Palavra Passe
														
													
													Consulta de NIF
											
										
										Espaço do Contribuinte
										
											Ajuda
											
												Perguntas Frequentes
												Fale Connosco
												Contactos
                                                
                                                    
                                                        Guias Rápidos
                                                    
													
														

															
																
																	Portal do Contribuinte
																
															
															
																
																	Acordos de Dupla Tributação (ADT)
																
															
															
																
																	IVA
																
															
															
																
																	IEC
																
															





															
																
																	II
																
															
															
																
																	IP
																
															
															
																
																	IVM
																
															
															
																
																	Emissão de Facturas
																
															
															
																
																	Cadastro de Património
																
															
															
																
																	Perguntas Frequentes
																
															
														
													
                                                
												
													
														Solicitar Novo Acesso
													
												
											
										
										
											Biblioteca Virtual
											
												Legislação
												
													Modelos
												

												
													
														Formulários
													
													
														
															
																Não Editáveis
															
														
														
															
																Editáveis
															
														
													
												
												Impostos e Taxas
												Notícias
												Videos
												Links Úteis
											
										
									
								
							
						
					
				

				
					
					
					sdfsd
					
					
				

			

		
		
			var NIF = localStorage.getItem('personNif')
			var NIF_COMPOSTO = localStorage.getItem('NIF_COMPOSTO')

			if (NIF !== NIF_COMPOSTO) {
				console.log('É diferente')
				if (location.pathname !== '/notificacoes-comunicados') {
					if (location.pathname !== '/extrato-de-conta-corrente') {
						console.log('Alterou')
						localStorage.setItem('NIF_COMPOSTO', NIF)
					}
				}
			}
		


	
	
		
				
					Menu de Serviços
							
								
									Cadastro de Contribuinte
								
									
										Consultar Cadastro
										

										Alterar Cadastro
										
										Criar Modelo 6
										
										
											Emitir Comprovativo
										
											Consultar Modelo 6
									
											
												
													Emitir Certificado
												
												
													
														
															Residência Fiscal
														
													
												
											
									
								
							

                        
                            
                                Património
                            
                            
                            
                                
                                    Consultar
                                
                            

                            
                                
                                    Pré-Cadastro
                                
                                

                                    
                                        
                                            Inscrever
                                        

                                        
                                            
                                                
                                                    Prédio Urbano
                                                
                                                
                                                    
                                                        
                                                            Propriedade Horizontal
                                                        
                                                    
                                                    
                                                        
                                                            Propriedade Total
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    Prédio Rustico
                                                
                                            
                                            
                                                
                                                    Fracção
                                                
                                            

                                        
                                    

                                    
                                        
                                            Consultar
                                        
                                    
                                
                            
							
								
									Gestão Contrato de Arrendamento
								
									Consultar Contrato de Arrendamento
									Registar Contrato de Arrendamento
								
							
                            
                        
                            
                                
                                    Liquidações
                                
                                    Liquidar Imposto
                                    
                                    Consultar Liquidações
                                
                            
                            
                                
                                    Pagamentos
                                
                                    
                                        Histórico de Pagamentos
                                    
									
                                    
                                        Verificar Nº NL / RP
                                    






								
							
						
							Certidão de Não Devedor
							
								Solicitar
								Verificar
							
						
								
									
										Verificar Certificado
									
									
										
											
												Residência Fiscal
											
										





									
								
							
								
									Conta Corrente
								
									
										Extracto de Conta Corrente
									
								
							
						
							Declarações
							
								
									II
									
								
										
											Consultar
											
											
												
													
														Declaração Anual
													
												
													
														
															Liquidação Provisória
														
													
													
														
															Mapa Retenção Fonte
														
													
													
														
															Valores Retidos
														
													
											
										
										
											Entregar
											
											
												
													
														Declaração Anual
													
												
													
														
															Liquidação Provisória
														
													
													
														
															Mapa Retenção Fonte
														
													
											
										
									
								

								
									
										IRT
									
										
												IRT
										
									
								


								
									IEC
									
										
											Consultar
											
												
													
														
															Declaração de IEC
														
													
												
											
										
										
											Entregar
											
												
													
														
															Declaração de IEC
														
													
												
											
										
									
								
								
									IVA
									
										
											Consultar
											
												
													
															
																Declaração de IVA
															
													
												
												
													
														
															Mapa de Fornecedores
														
													
												
											
										
										
											Entregar
											
												
													
															
																Declaração de IVA
															
													
												
												
													
														
															Mapa de Fornecedores
														
													
												
											
										
									
								

								
									
										Gestão de Facturação
									
										
												Submissão do SAF-T
												Facturas Manuais
										
									
								
								
									
										IVM
									
										
												Submissão do IVM
										
									
								






















							
						

						
							
									Notificações e Comunicações
							
						

						
							
									Verificar
							
						

						
							
								Reembolsos
							
								
										Consultar
									

									
										
											Missões Diplomáticas
										
											
												Entregar
											
											
												Consultar
											
										
								
							
						

							
							
								
									Produtores de Software
								
									
											Submissão do Modelo 8
											Consultar Certificado
									
								
							

							
								
									Gráficas e Tipografias
								
									
											Solicitação de Licenciamento
											Consultar Factura
									
								
							

							
									Emissão de Facturas
							
							
								
									Gestão do Utilizador
								
									
										Alterar Palavra Passe
									
								
							
				
		

		
			

                

                    Entregar Mapa de Retenção na Fonte de Imposto Industrial

                    

                        Geral

                        Detalhes do Contribuinte

                        Número Fiscal
                            
                            
                        
                        Nome/Conta
                            DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
                            
                        
                        Informações do Mapa
                        Número do Documento
                            
                            Documento Anterior
                            
                            
                        
                        Tipo de Documento
                            
                            
                        
                            Motivo
                                Iniciativa do ContribuinteNotificaçãoIniciativa do ContribuinteIniciativa do ContribuinteNotificação
                                
                            
                            
                                
                                
                                
                                
                            
                    

                    
                        Detalhes do Imposto
                        Tipo do Imposto
                            
                            
                        
                        Ano
                            
                            
                        
                        Período
                            
                            
                        
                    

                    
                        Dados do Prestador de Serviço / Detalhes da Factura
                            Adicionar Linha
                            
                            Upload de Ficheiro
                            
                            Apagar Dados
                            
                            Template do Ficheiro
                            NIF EM ANGOLA?NIFNOMEN° DECLARAÇÃO DE CONFORMIDADESERVIÇOS PRESTADOS AO SECTOR PETROLÍFERO?Nº FACTURADESCRIÇÃO SERVIÇODATA FACTURADATA PAGAMENTOVALOR DA FACTURAVALOR PAGOVALOR SUJEITO A RETENÇÃOTAXAIMPOSTO RETIDOEditarRemoverSIM5301003682ORGANIZACOES DIVUILANÃOdsdsadadaasdas02/02/202302/02/2023Kz 100 000,00Kz 1 000,00Kz 100,00 6.5 %Kz 6,50EditarRemoverSIM5401144440A MUSE - A MUNDIAL SEGUROS, S.A.NÃOFTMA2023/222233TESTES DE SERVIÇO08/02/202309/02/2023Kz 20 000,00Kz 20 000,00Kz 500,00 6.5 %Kz 32,50FP1NE
                    

                    
                        Valores da Liquidação (Kz)
                        
                            Total de Imposto
                        
                        Imposto a Pagar
                            
                            
                        
                    

                        Gravar
                        

                Eliminar Mapa de Retenção na Fonte de II
                    Confirmar eliminação do mapa:
                    
                    Número Fiscal
                        
                        
                    
                    Nome/Conta
                        DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
                        
                    
                    Ano e Período
                        
                        
                        
                        
                    
                    Número do Documento
                        
                        
                    
                    Motivo
                        Erro imputável ao contribuinte
                        
                    
                    ConfirmarVoltar
                    Emissão de Mapa de Retenção na Fonte de Imposto IndustrialTipo de Ficheiro*
                            PDFEXCELPDF
                            

                        Tipo de Mapa*
                            GeralPor PrestadorGeral
                            
                        

                    ImprimirVoltar
                    
			
		
	

                
                    
                        Aviso
                    
                

            
                
                    Caro Contribuinte,
                    Os seus dados de cadastro encontram-se desactualizados. Deverá proceder à sua actualização na opção
                    Cadastro de Contribuinte -> Actualizar Cadastro.
                    Persistindo esta situação, poderá ser limitada a sua utilização do Portal, em nome próprio
                    ou através dos seus representantes, passando a disponibilizar somente consultas e actualização ao Cadastro.
                    
                     Resta 0 dia
                    
                

            


            
                    Fechar
            
		

	
		
			
			
				
					
                    
                    
                    
					AGT
				
				
				 	Fale Connosco
					Contactos
					Perguntas Frequentes
					Verificar Nota de Liquidação
				
			
		
	
    
    	
    	
    	
    	
    	
    	
               

	
	
                
                    
                        
                    
                

            
                
                
                
                
            

            Não mostrar novamente
                    OK
            
Nova Declaração de Imposto Industrial
			Caro Contribuinte, seleccione abaixo o tipo de declaração de Imposto Industrial de acordo com o seu grupo de tributação.
				Modelo 1 - Regime Geral
				
				Modelo 1 - Regime Geral (Sector Financeiro)
				

				Modelo de Declaração de Regime Simplificado sem Contabilidade
				
			    Alerta de Erros Upload de Ficheiro
                    
                        
                            Caro contribuinte, foram detectados erros nos valores informados no ficheiro.
                            
                        

                        
                            MensagemNenhum registo encontradoFPNE
                            
                        
                    Alerta de Valores
                    
                        
                            Caro contribuinte, foram detectadas divergências entre os valores informados no ficheiro e os valores calculados pelo sistema.
                                Clica em Autorizar para aplicar a correcção automática ou Recusar para corrigir o ficheiro.
                            
                        
                        
                            AutorizarRecusar
                            
                        

                        
                            Nº OrdemCampoValor DeclaradoValor Apurado pela AGTNenhum registo encontradoFPNE
                            
                        
                    Selecção de Declaração em Conformidade

                    
                        
                        

                        
                            
                                Número Fiscal*
                                    
                                    
                                
                                    Nome/Conta*
                                         
                                        
                                    
                            
                        

                        
                            #AcçãoNúmero DeclaraçãoAnoPeríodoCódigo de ImpostoNenhum registo encontrado
                            
                        

                        
                            Voltar
                            
                        

                    Dados do Prestador de Serviço / Detalhes da Factura
                    
                        
                        

                        Identificação do Prestador de Serviço
                        
                        
                            
                                
                                    NIF em Angola?
                                        SimNãoSimSimNão
                                        
                                    
                                            Número Fiscal*
                                                
                                                
                                            
                                            Nome/Conta*
                                                 
                                                
                                            
                                        N° Declaração de Conformidade
                                            
                                            
                                            ui-button
                                            
                                        
                                
                            
                        
                        Detalhes da Factura
                        
                        
                            
                                

                                    Sector de Prestação de Serviço Petrolífero?
                                        NãoSimNãoNãoSim
                                        
                                    

                                    Número da Factura*
                                        
                                        
                                    
                                    Descrição do Serviço*
                                        
                                        
                                    
                                    Data da Factura*
                                        
                                        
                                    
                                    Data do Pagamento*
                                        
                                        
                                    
                                    Valor da Factura
                                        
                                        
                                    
                                    Valor Pago
                                        
                                        
                                    
                                    Valor Sujeito a Retenção
                                        
                                        
                                    
                                
                            
                        
                        
                            
                                
                                    Taxa
                                        
                                        
                                    
                                    Imposto Retido
                                        
                                        
                                    
                                
                            
                        
                        
                            AdicionarSair
                            
                        
                    EXCELCSVXMLUpload de Ficheiro
                    
                        
                            Upload de Ficheiro*
                                EXCELCSVXMLEXCEL
                                
                            
                        
                        
                            Importar Mapa
                            
                        
                        
                            
                                
                                    Após seleccionar o ficheiro aguarde esta janela fechar automaticamente ao término da importação. Este processo pode demorar alguns minutos.
                                
                                
                                
                                    SeleccionarVoltar
                                    
                                
                            
                        
                    EXCELCSVXMLTemplate de Ficheiro de Dados do Prestador de Serviço
                    
                        
                            Tipo de Ficheiro*
                                EXCELCSVXMLEXCEL
                                
                            
                        
                        DownloadVoltar
                        
                    PDFEXCELGeralPor Prestadorid(&quot;dialog_modal&quot;)Prezado contribuinte, não utilizar vírgula ( , ) ou apóstrofe ( ' ) ou ponto e vírgula ( ; )</value>
      <webElementGuid>a4375cd4-a428-41dd-968e-220a10886c92</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>8b019eb2-14be-46b5-8d30-80ff86169f68</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>8d97393c-a84f-4dbb-92ba-afd3278aaafb</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;


			
			
				
					
						
							
							Encerrar Sessão
						
						
							
								
							
							
								Deseja continuar na sessão do Portal do Contribuinte?
								Continuar
								Sair
							

						

					
				
			

			
				
					

					
					
					
					

					

					
						
							

							

						
						
							19:22
						

						
							
								
									
										Toggle navigation
									
								

									
											
												
													DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
													
												
												
													NIF TESTE PROJECTO SIGT
													MUDAR DE CONTA
												
											
									
							
						

						
						
								Sair
						
						
					

				

				

					
						

							
								Portal do Contribuinte
							

							
							
								
									
										Toggle navigation
										
										
										
									
								
								
									
										Início
										
											Serviços
											
													Notificações e Comunicações 
													
														Cadastro de Contribuinte
														
															Consultar Cadastro 
															Alterar Cadastro 
															Criar Modelo 6 
															Emitir Comprovativo
															Consultar Modelo 6
															Emitir Certificado
														
													
												
                                                        Património
                                                        
                                                            
                                                                Consultar Património
                                                                
                                                                    Pré-Cadastro Electrónico
                                                                    
                                                                        Cadastrar Fracção
                                                                        Consultar / Cadastrar Património
                                                                    
                                                                
                                                            
                                                        
                                                    

													
														Liquidações
														
															Liquidar Imposto 
															Consultar Liquidações
														
													
													
														Pagamentos
														
															Histórico de Pagamentos
															
															Verificar Nº NL / RP
														
													

													
														Certidão de Não Devedor
														
															Solicitar
															Verificar
														
													

													
														Conta Corrente
														
															
																Extracto de Conta Corrente
															
																
																	Certidão de não Devedor
																
																i

														
													
													
														Declarações
														
																
																	II
																	
																		
																			Consultar

																			
																				Declaração Anual
																			

																		
																		
																			Entregar

																			
																				Declaração Anual
																			

																		
																	
																

															
																IRT
																
																	
																		IRT
																
															

															
																IEC
																
																	
																		Consultar

																		
																			Declaração de IEC
																		

																	
																	
																		Entregar

																		
																			Declaração de IEC
																		

																	
																
															
															
																IVA
																
																	
																		Consultar

																		
																			Declaração de IVA
																			Mapa de Fornecedores
																		

																	
																	
																		Entregar

																		
																			Declaração de IVA
																			Mapa de Fornecedores
																		

																	
																
															

















															
																Gestão de Facturação
																
																	
																		Submissão do SAF-T
																	
																		Facturas Manuais
																
															
															
																IVM
																
																	
																		Submissão do IVM
																
															

														
													
													
														Reembolsos
														
															IVA
														
													
													
														Produtores de software
														
															
																	Submissão do Modelo 8
																	Consultar Certificado
															
														
													

													
														Verificar Certificado
														
															Residência Fiscal
														
													

													
														Gráficas e Tipografias
														
															
																	Solicitação de Licenciamento
																	Consultar Factura
															
														
													

													
															Emissão de Facturas
													

													
														Gestão do Utilizador
														
															Alterar Palavra Passe
														
													
													Consulta de NIF
											
										
										Espaço do Contribuinte
										
											Ajuda
											
												Perguntas Frequentes
												Fale Connosco
												Contactos
                                                
                                                    
                                                        Guias Rápidos
                                                    
													
														

															
																
																	Portal do Contribuinte
																
															
															
																
																	Acordos de Dupla Tributação (ADT)
																
															
															
																
																	IVA
																
															
															
																
																	IEC
																
															





															
																
																	II
																
															
															
																
																	IP
																
															
															
																
																	IVM
																
															
															
																
																	Emissão de Facturas
																
															
															
																
																	Cadastro de Património
																
															
															
																
																	Perguntas Frequentes
																
															
														
													
                                                
												
													
														Solicitar Novo Acesso
													
												
											
										
										
											Biblioteca Virtual
											
												Legislação
												
													Modelos
												

												
													
														Formulários
													
													
														
															
																Não Editáveis
															
														
														
															
																Editáveis
															
														
													
												
												Impostos e Taxas
												Notícias
												Videos
												Links Úteis
											
										
									
								
							
						
					
				

				
					
					
					sdfsd
					
					
				

			

		
		
			var NIF = localStorage.getItem(&quot; , &quot;'&quot; , &quot;personNif&quot; , &quot;'&quot; , &quot;)
			var NIF_COMPOSTO = localStorage.getItem(&quot; , &quot;'&quot; , &quot;NIF_COMPOSTO&quot; , &quot;'&quot; , &quot;)

			if (NIF !== NIF_COMPOSTO) {
				console.log(&quot; , &quot;'&quot; , &quot;É diferente&quot; , &quot;'&quot; , &quot;)
				if (location.pathname !== &quot; , &quot;'&quot; , &quot;/notificacoes-comunicados&quot; , &quot;'&quot; , &quot;) {
					if (location.pathname !== &quot; , &quot;'&quot; , &quot;/extrato-de-conta-corrente&quot; , &quot;'&quot; , &quot;) {
						console.log(&quot; , &quot;'&quot; , &quot;Alterou&quot; , &quot;'&quot; , &quot;)
						localStorage.setItem(&quot; , &quot;'&quot; , &quot;NIF_COMPOSTO&quot; , &quot;'&quot; , &quot;, NIF)
					}
				}
			}
		


	
	
		
				
					Menu de Serviços
							
								
									Cadastro de Contribuinte
								
									
										Consultar Cadastro
										

										Alterar Cadastro
										
										Criar Modelo 6
										
										
											Emitir Comprovativo
										
											Consultar Modelo 6
									
											
												
													Emitir Certificado
												
												
													
														
															Residência Fiscal
														
													
												
											
									
								
							

                        
                            
                                Património
                            
                            
                            
                                
                                    Consultar
                                
                            

                            
                                
                                    Pré-Cadastro
                                
                                

                                    
                                        
                                            Inscrever
                                        

                                        
                                            
                                                
                                                    Prédio Urbano
                                                
                                                
                                                    
                                                        
                                                            Propriedade Horizontal
                                                        
                                                    
                                                    
                                                        
                                                            Propriedade Total
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    Prédio Rustico
                                                
                                            
                                            
                                                
                                                    Fracção
                                                
                                            

                                        
                                    

                                    
                                        
                                            Consultar
                                        
                                    
                                
                            
							
								
									Gestão Contrato de Arrendamento
								
									Consultar Contrato de Arrendamento
									Registar Contrato de Arrendamento
								
							
                            
                        
                            
                                
                                    Liquidações
                                
                                    Liquidar Imposto
                                    
                                    Consultar Liquidações
                                
                            
                            
                                
                                    Pagamentos
                                
                                    
                                        Histórico de Pagamentos
                                    
									
                                    
                                        Verificar Nº NL / RP
                                    






								
							
						
							Certidão de Não Devedor
							
								Solicitar
								Verificar
							
						
								
									
										Verificar Certificado
									
									
										
											
												Residência Fiscal
											
										





									
								
							
								
									Conta Corrente
								
									
										Extracto de Conta Corrente
									
								
							
						
							Declarações
							
								
									II
									
								
										
											Consultar
											
											
												
													
														Declaração Anual
													
												
													
														
															Liquidação Provisória
														
													
													
														
															Mapa Retenção Fonte
														
													
													
														
															Valores Retidos
														
													
											
										
										
											Entregar
											
											
												
													
														Declaração Anual
													
												
													
														
															Liquidação Provisória
														
													
													
														
															Mapa Retenção Fonte
														
													
											
										
									
								

								
									
										IRT
									
										
												IRT
										
									
								


								
									IEC
									
										
											Consultar
											
												
													
														
															Declaração de IEC
														
													
												
											
										
										
											Entregar
											
												
													
														
															Declaração de IEC
														
													
												
											
										
									
								
								
									IVA
									
										
											Consultar
											
												
													
															
																Declaração de IVA
															
													
												
												
													
														
															Mapa de Fornecedores
														
													
												
											
										
										
											Entregar
											
												
													
															
																Declaração de IVA
															
													
												
												
													
														
															Mapa de Fornecedores
														
													
												
											
										
									
								

								
									
										Gestão de Facturação
									
										
												Submissão do SAF-T
												Facturas Manuais
										
									
								
								
									
										IVM
									
										
												Submissão do IVM
										
									
								






















							
						

						
							
									Notificações e Comunicações
							
						

						
							
									Verificar
							
						

						
							
								Reembolsos
							
								
										Consultar
									

									
										
											Missões Diplomáticas
										
											
												Entregar
											
											
												Consultar
											
										
								
							
						

							
							
								
									Produtores de Software
								
									
											Submissão do Modelo 8
											Consultar Certificado
									
								
							

							
								
									Gráficas e Tipografias
								
									
											Solicitação de Licenciamento
											Consultar Factura
									
								
							

							
									Emissão de Facturas
							
							
								
									Gestão do Utilizador
								
									
										Alterar Palavra Passe
									
								
							
				
		

		
			

                

                    Entregar Mapa de Retenção na Fonte de Imposto Industrial

                    

                        Geral

                        Detalhes do Contribuinte

                        Número Fiscal
                            
                            
                        
                        Nome/Conta
                            DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
                            
                        
                        Informações do Mapa
                        Número do Documento
                            
                            Documento Anterior
                            
                            
                        
                        Tipo de Documento
                            
                            
                        
                            Motivo
                                Iniciativa do ContribuinteNotificaçãoIniciativa do ContribuinteIniciativa do ContribuinteNotificação
                                
                            
                            
                                
                                
                                
                                
                            
                    

                    
                        Detalhes do Imposto
                        Tipo do Imposto
                            
                            
                        
                        Ano
                            
                            
                        
                        Período
                            
                            
                        
                    

                    
                        Dados do Prestador de Serviço / Detalhes da Factura
                            Adicionar Linha
                            
                            Upload de Ficheiro
                            
                            Apagar Dados
                            
                            Template do Ficheiro
                            NIF EM ANGOLA?NIFNOMEN° DECLARAÇÃO DE CONFORMIDADESERVIÇOS PRESTADOS AO SECTOR PETROLÍFERO?Nº FACTURADESCRIÇÃO SERVIÇODATA FACTURADATA PAGAMENTOVALOR DA FACTURAVALOR PAGOVALOR SUJEITO A RETENÇÃOTAXAIMPOSTO RETIDOEditarRemoverSIM5301003682ORGANIZACOES DIVUILANÃOdsdsadadaasdas02/02/202302/02/2023Kz 100 000,00Kz 1 000,00Kz 100,00 6.5 %Kz 6,50EditarRemoverSIM5401144440A MUSE - A MUNDIAL SEGUROS, S.A.NÃOFTMA2023/222233TESTES DE SERVIÇO08/02/202309/02/2023Kz 20 000,00Kz 20 000,00Kz 500,00 6.5 %Kz 32,50FP1NE
                    

                    
                        Valores da Liquidação (Kz)
                        
                            Total de Imposto
                        
                        Imposto a Pagar
                            
                            
                        
                    

                        Gravar
                        

                Eliminar Mapa de Retenção na Fonte de II
                    Confirmar eliminação do mapa:
                    
                    Número Fiscal
                        
                        
                    
                    Nome/Conta
                        DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
                        
                    
                    Ano e Período
                        
                        
                        
                        
                    
                    Número do Documento
                        
                        
                    
                    Motivo
                        Erro imputável ao contribuinte
                        
                    
                    ConfirmarVoltar
                    Emissão de Mapa de Retenção na Fonte de Imposto IndustrialTipo de Ficheiro*
                            PDFEXCELPDF
                            

                        Tipo de Mapa*
                            GeralPor PrestadorGeral
                            
                        

                    ImprimirVoltar
                    
			
		
	

                
                    
                        Aviso
                    
                

            
                
                    Caro Contribuinte,
                    Os seus dados de cadastro encontram-se desactualizados. Deverá proceder à sua actualização na opção
                    Cadastro de Contribuinte -> Actualizar Cadastro.
                    Persistindo esta situação, poderá ser limitada a sua utilização do Portal, em nome próprio
                    ou através dos seus representantes, passando a disponibilizar somente consultas e actualização ao Cadastro.
                    
                     Resta 0 dia
                    
                

            


            
                    Fechar
            
		

	
		
			
			
				
					
                    
                    
                    
					AGT
				
				
				 	Fale Connosco
					Contactos
					Perguntas Frequentes
					Verificar Nota de Liquidação
				
			
		
	
    
    	
    	
    	
    	
    	
    	
               

	
	
                
                    
                        
                    
                

            
                
                
                
                
            

            Não mostrar novamente
                    OK
            
Nova Declaração de Imposto Industrial
			Caro Contribuinte, seleccione abaixo o tipo de declaração de Imposto Industrial de acordo com o seu grupo de tributação.
				Modelo 1 - Regime Geral
				
				Modelo 1 - Regime Geral (Sector Financeiro)
				

				Modelo de Declaração de Regime Simplificado sem Contabilidade
				
			    Alerta de Erros Upload de Ficheiro
                    
                        
                            Caro contribuinte, foram detectados erros nos valores informados no ficheiro.
                            
                        

                        
                            MensagemNenhum registo encontradoFPNE
                            
                        
                    Alerta de Valores
                    
                        
                            Caro contribuinte, foram detectadas divergências entre os valores informados no ficheiro e os valores calculados pelo sistema.
                                Clica em Autorizar para aplicar a correcção automática ou Recusar para corrigir o ficheiro.
                            
                        
                        
                            AutorizarRecusar
                            
                        

                        
                            Nº OrdemCampoValor DeclaradoValor Apurado pela AGTNenhum registo encontradoFPNE
                            
                        
                    Selecção de Declaração em Conformidade

                    
                        
                        

                        
                            
                                Número Fiscal*
                                    
                                    
                                
                                    Nome/Conta*
                                         
                                        
                                    
                            
                        

                        
                            #AcçãoNúmero DeclaraçãoAnoPeríodoCódigo de ImpostoNenhum registo encontrado
                            
                        

                        
                            Voltar
                            
                        

                    Dados do Prestador de Serviço / Detalhes da Factura
                    
                        
                        

                        Identificação do Prestador de Serviço
                        
                        
                            
                                
                                    NIF em Angola?
                                        SimNãoSimSimNão
                                        
                                    
                                            Número Fiscal*
                                                
                                                
                                            
                                            Nome/Conta*
                                                 
                                                
                                            
                                        N° Declaração de Conformidade
                                            
                                            
                                            ui-button
                                            
                                        
                                
                            
                        
                        Detalhes da Factura
                        
                        
                            
                                

                                    Sector de Prestação de Serviço Petrolífero?
                                        NãoSimNãoNãoSim
                                        
                                    

                                    Número da Factura*
                                        
                                        
                                    
                                    Descrição do Serviço*
                                        
                                        
                                    
                                    Data da Factura*
                                        
                                        
                                    
                                    Data do Pagamento*
                                        
                                        
                                    
                                    Valor da Factura
                                        
                                        
                                    
                                    Valor Pago
                                        
                                        
                                    
                                    Valor Sujeito a Retenção
                                        
                                        
                                    
                                
                            
                        
                        
                            
                                
                                    Taxa
                                        
                                        
                                    
                                    Imposto Retido
                                        
                                        
                                    
                                
                            
                        
                        
                            AdicionarSair
                            
                        
                    EXCELCSVXMLUpload de Ficheiro
                    
                        
                            Upload de Ficheiro*
                                EXCELCSVXMLEXCEL
                                
                            
                        
                        
                            Importar Mapa
                            
                        
                        
                            
                                
                                    Após seleccionar o ficheiro aguarde esta janela fechar automaticamente ao término da importação. Este processo pode demorar alguns minutos.
                                
                                
                                
                                    SeleccionarVoltar
                                    
                                
                            
                        
                    EXCELCSVXMLTemplate de Ficheiro de Dados do Prestador de Serviço
                    
                        
                            Tipo de Ficheiro*
                                EXCELCSVXMLEXCEL
                                
                            
                        
                        DownloadVoltar
                        
                    PDFEXCELGeralPor Prestadorid(&quot;dialog_modal&quot;)Prezado contribuinte, não utilizar vírgula ( , ) ou apóstrofe ( &quot; , &quot;'&quot; , &quot; ) ou ponto e vírgula ( ; )&quot;) or . = concat(&quot;


			
			
				
					
						
							
							Encerrar Sessão
						
						
							
								
							
							
								Deseja continuar na sessão do Portal do Contribuinte?
								Continuar
								Sair
							

						

					
				
			

			
				
					

					
					
					
					

					

					
						
							

							

						
						
							19:22
						

						
							
								
									
										Toggle navigation
									
								

									
											
												
													DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
													
												
												
													NIF TESTE PROJECTO SIGT
													MUDAR DE CONTA
												
											
									
							
						

						
						
								Sair
						
						
					

				

				

					
						

							
								Portal do Contribuinte
							

							
							
								
									
										Toggle navigation
										
										
										
									
								
								
									
										Início
										
											Serviços
											
													Notificações e Comunicações 
													
														Cadastro de Contribuinte
														
															Consultar Cadastro 
															Alterar Cadastro 
															Criar Modelo 6 
															Emitir Comprovativo
															Consultar Modelo 6
															Emitir Certificado
														
													
												
                                                        Património
                                                        
                                                            
                                                                Consultar Património
                                                                
                                                                    Pré-Cadastro Electrónico
                                                                    
                                                                        Cadastrar Fracção
                                                                        Consultar / Cadastrar Património
                                                                    
                                                                
                                                            
                                                        
                                                    

													
														Liquidações
														
															Liquidar Imposto 
															Consultar Liquidações
														
													
													
														Pagamentos
														
															Histórico de Pagamentos
															
															Verificar Nº NL / RP
														
													

													
														Certidão de Não Devedor
														
															Solicitar
															Verificar
														
													

													
														Conta Corrente
														
															
																Extracto de Conta Corrente
															
																
																	Certidão de não Devedor
																
																i

														
													
													
														Declarações
														
																
																	II
																	
																		
																			Consultar

																			
																				Declaração Anual
																			

																		
																		
																			Entregar

																			
																				Declaração Anual
																			

																		
																	
																

															
																IRT
																
																	
																		IRT
																
															

															
																IEC
																
																	
																		Consultar

																		
																			Declaração de IEC
																		

																	
																	
																		Entregar

																		
																			Declaração de IEC
																		

																	
																
															
															
																IVA
																
																	
																		Consultar

																		
																			Declaração de IVA
																			Mapa de Fornecedores
																		

																	
																	
																		Entregar

																		
																			Declaração de IVA
																			Mapa de Fornecedores
																		

																	
																
															

















															
																Gestão de Facturação
																
																	
																		Submissão do SAF-T
																	
																		Facturas Manuais
																
															
															
																IVM
																
																	
																		Submissão do IVM
																
															

														
													
													
														Reembolsos
														
															IVA
														
													
													
														Produtores de software
														
															
																	Submissão do Modelo 8
																	Consultar Certificado
															
														
													

													
														Verificar Certificado
														
															Residência Fiscal
														
													

													
														Gráficas e Tipografias
														
															
																	Solicitação de Licenciamento
																	Consultar Factura
															
														
													

													
															Emissão de Facturas
													

													
														Gestão do Utilizador
														
															Alterar Palavra Passe
														
													
													Consulta de NIF
											
										
										Espaço do Contribuinte
										
											Ajuda
											
												Perguntas Frequentes
												Fale Connosco
												Contactos
                                                
                                                    
                                                        Guias Rápidos
                                                    
													
														

															
																
																	Portal do Contribuinte
																
															
															
																
																	Acordos de Dupla Tributação (ADT)
																
															
															
																
																	IVA
																
															
															
																
																	IEC
																
															





															
																
																	II
																
															
															
																
																	IP
																
															
															
																
																	IVM
																
															
															
																
																	Emissão de Facturas
																
															
															
																
																	Cadastro de Património
																
															
															
																
																	Perguntas Frequentes
																
															
														
													
                                                
												
													
														Solicitar Novo Acesso
													
												
											
										
										
											Biblioteca Virtual
											
												Legislação
												
													Modelos
												

												
													
														Formulários
													
													
														
															
																Não Editáveis
															
														
														
															
																Editáveis
															
														
													
												
												Impostos e Taxas
												Notícias
												Videos
												Links Úteis
											
										
									
								
							
						
					
				

				
					
					
					sdfsd
					
					
				

			

		
		
			var NIF = localStorage.getItem(&quot; , &quot;'&quot; , &quot;personNif&quot; , &quot;'&quot; , &quot;)
			var NIF_COMPOSTO = localStorage.getItem(&quot; , &quot;'&quot; , &quot;NIF_COMPOSTO&quot; , &quot;'&quot; , &quot;)

			if (NIF !== NIF_COMPOSTO) {
				console.log(&quot; , &quot;'&quot; , &quot;É diferente&quot; , &quot;'&quot; , &quot;)
				if (location.pathname !== &quot; , &quot;'&quot; , &quot;/notificacoes-comunicados&quot; , &quot;'&quot; , &quot;) {
					if (location.pathname !== &quot; , &quot;'&quot; , &quot;/extrato-de-conta-corrente&quot; , &quot;'&quot; , &quot;) {
						console.log(&quot; , &quot;'&quot; , &quot;Alterou&quot; , &quot;'&quot; , &quot;)
						localStorage.setItem(&quot; , &quot;'&quot; , &quot;NIF_COMPOSTO&quot; , &quot;'&quot; , &quot;, NIF)
					}
				}
			}
		


	
	
		
				
					Menu de Serviços
							
								
									Cadastro de Contribuinte
								
									
										Consultar Cadastro
										

										Alterar Cadastro
										
										Criar Modelo 6
										
										
											Emitir Comprovativo
										
											Consultar Modelo 6
									
											
												
													Emitir Certificado
												
												
													
														
															Residência Fiscal
														
													
												
											
									
								
							

                        
                            
                                Património
                            
                            
                            
                                
                                    Consultar
                                
                            

                            
                                
                                    Pré-Cadastro
                                
                                

                                    
                                        
                                            Inscrever
                                        

                                        
                                            
                                                
                                                    Prédio Urbano
                                                
                                                
                                                    
                                                        
                                                            Propriedade Horizontal
                                                        
                                                    
                                                    
                                                        
                                                            Propriedade Total
                                                        
                                                    
                                                
                                            

                                            
                                                
                                                    Prédio Rustico
                                                
                                            
                                            
                                                
                                                    Fracção
                                                
                                            

                                        
                                    

                                    
                                        
                                            Consultar
                                        
                                    
                                
                            
							
								
									Gestão Contrato de Arrendamento
								
									Consultar Contrato de Arrendamento
									Registar Contrato de Arrendamento
								
							
                            
                        
                            
                                
                                    Liquidações
                                
                                    Liquidar Imposto
                                    
                                    Consultar Liquidações
                                
                            
                            
                                
                                    Pagamentos
                                
                                    
                                        Histórico de Pagamentos
                                    
									
                                    
                                        Verificar Nº NL / RP
                                    






								
							
						
							Certidão de Não Devedor
							
								Solicitar
								Verificar
							
						
								
									
										Verificar Certificado
									
									
										
											
												Residência Fiscal
											
										





									
								
							
								
									Conta Corrente
								
									
										Extracto de Conta Corrente
									
								
							
						
							Declarações
							
								
									II
									
								
										
											Consultar
											
											
												
													
														Declaração Anual
													
												
													
														
															Liquidação Provisória
														
													
													
														
															Mapa Retenção Fonte
														
													
													
														
															Valores Retidos
														
													
											
										
										
											Entregar
											
											
												
													
														Declaração Anual
													
												
													
														
															Liquidação Provisória
														
													
													
														
															Mapa Retenção Fonte
														
													
											
										
									
								

								
									
										IRT
									
										
												IRT
										
									
								


								
									IEC
									
										
											Consultar
											
												
													
														
															Declaração de IEC
														
													
												
											
										
										
											Entregar
											
												
													
														
															Declaração de IEC
														
													
												
											
										
									
								
								
									IVA
									
										
											Consultar
											
												
													
															
																Declaração de IVA
															
													
												
												
													
														
															Mapa de Fornecedores
														
													
												
											
										
										
											Entregar
											
												
													
															
																Declaração de IVA
															
													
												
												
													
														
															Mapa de Fornecedores
														
													
												
											
										
									
								

								
									
										Gestão de Facturação
									
										
												Submissão do SAF-T
												Facturas Manuais
										
									
								
								
									
										IVM
									
										
												Submissão do IVM
										
									
								






















							
						

						
							
									Notificações e Comunicações
							
						

						
							
									Verificar
							
						

						
							
								Reembolsos
							
								
										Consultar
									

									
										
											Missões Diplomáticas
										
											
												Entregar
											
											
												Consultar
											
										
								
							
						

							
							
								
									Produtores de Software
								
									
											Submissão do Modelo 8
											Consultar Certificado
									
								
							

							
								
									Gráficas e Tipografias
								
									
											Solicitação de Licenciamento
											Consultar Factura
									
								
							

							
									Emissão de Facturas
							
							
								
									Gestão do Utilizador
								
									
										Alterar Palavra Passe
									
								
							
				
		

		
			

                

                    Entregar Mapa de Retenção na Fonte de Imposto Industrial

                    

                        Geral

                        Detalhes do Contribuinte

                        Número Fiscal
                            
                            
                        
                        Nome/Conta
                            DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
                            
                        
                        Informações do Mapa
                        Número do Documento
                            
                            Documento Anterior
                            
                            
                        
                        Tipo de Documento
                            
                            
                        
                            Motivo
                                Iniciativa do ContribuinteNotificaçãoIniciativa do ContribuinteIniciativa do ContribuinteNotificação
                                
                            
                            
                                
                                
                                
                                
                            
                    

                    
                        Detalhes do Imposto
                        Tipo do Imposto
                            
                            
                        
                        Ano
                            
                            
                        
                        Período
                            
                            
                        
                    

                    
                        Dados do Prestador de Serviço / Detalhes da Factura
                            Adicionar Linha
                            
                            Upload de Ficheiro
                            
                            Apagar Dados
                            
                            Template do Ficheiro
                            NIF EM ANGOLA?NIFNOMEN° DECLARAÇÃO DE CONFORMIDADESERVIÇOS PRESTADOS AO SECTOR PETROLÍFERO?Nº FACTURADESCRIÇÃO SERVIÇODATA FACTURADATA PAGAMENTOVALOR DA FACTURAVALOR PAGOVALOR SUJEITO A RETENÇÃOTAXAIMPOSTO RETIDOEditarRemoverSIM5301003682ORGANIZACOES DIVUILANÃOdsdsadadaasdas02/02/202302/02/2023Kz 100 000,00Kz 1 000,00Kz 100,00 6.5 %Kz 6,50EditarRemoverSIM5401144440A MUSE - A MUNDIAL SEGUROS, S.A.NÃOFTMA2023/222233TESTES DE SERVIÇO08/02/202309/02/2023Kz 20 000,00Kz 20 000,00Kz 500,00 6.5 %Kz 32,50FP1NE
                    

                    
                        Valores da Liquidação (Kz)
                        
                            Total de Imposto
                        
                        Imposto a Pagar
                            
                            
                        
                    

                        Gravar
                        

                Eliminar Mapa de Retenção na Fonte de II
                    Confirmar eliminação do mapa:
                    
                    Número Fiscal
                        
                        
                    
                    Nome/Conta
                        DIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDADIGITAL FOCUS - PROJECTOS EDUCATIVOS, LDA
                        
                    
                    Ano e Período
                        
                        
                        
                        
                    
                    Número do Documento
                        
                        
                    
                    Motivo
                        Erro imputável ao contribuinte
                        
                    
                    ConfirmarVoltar
                    Emissão de Mapa de Retenção na Fonte de Imposto IndustrialTipo de Ficheiro*
                            PDFEXCELPDF
                            

                        Tipo de Mapa*
                            GeralPor PrestadorGeral
                            
                        

                    ImprimirVoltar
                    
			
		
	

                
                    
                        Aviso
                    
                

            
                
                    Caro Contribuinte,
                    Os seus dados de cadastro encontram-se desactualizados. Deverá proceder à sua actualização na opção
                    Cadastro de Contribuinte -> Actualizar Cadastro.
                    Persistindo esta situação, poderá ser limitada a sua utilização do Portal, em nome próprio
                    ou através dos seus representantes, passando a disponibilizar somente consultas e actualização ao Cadastro.
                    
                     Resta 0 dia
                    
                

            


            
                    Fechar
            
		

	
		
			
			
				
					
                    
                    
                    
					AGT
				
				
				 	Fale Connosco
					Contactos
					Perguntas Frequentes
					Verificar Nota de Liquidação
				
			
		
	
    
    	
    	
    	
    	
    	
    	
               

	
	
                
                    
                        
                    
                

            
                
                
                
                
            

            Não mostrar novamente
                    OK
            
Nova Declaração de Imposto Industrial
			Caro Contribuinte, seleccione abaixo o tipo de declaração de Imposto Industrial de acordo com o seu grupo de tributação.
				Modelo 1 - Regime Geral
				
				Modelo 1 - Regime Geral (Sector Financeiro)
				

				Modelo de Declaração de Regime Simplificado sem Contabilidade
				
			    Alerta de Erros Upload de Ficheiro
                    
                        
                            Caro contribuinte, foram detectados erros nos valores informados no ficheiro.
                            
                        

                        
                            MensagemNenhum registo encontradoFPNE
                            
                        
                    Alerta de Valores
                    
                        
                            Caro contribuinte, foram detectadas divergências entre os valores informados no ficheiro e os valores calculados pelo sistema.
                                Clica em Autorizar para aplicar a correcção automática ou Recusar para corrigir o ficheiro.
                            
                        
                        
                            AutorizarRecusar
                            
                        

                        
                            Nº OrdemCampoValor DeclaradoValor Apurado pela AGTNenhum registo encontradoFPNE
                            
                        
                    Selecção de Declaração em Conformidade

                    
                        
                        

                        
                            
                                Número Fiscal*
                                    
                                    
                                
                                    Nome/Conta*
                                         
                                        
                                    
                            
                        

                        
                            #AcçãoNúmero DeclaraçãoAnoPeríodoCódigo de ImpostoNenhum registo encontrado
                            
                        

                        
                            Voltar
                            
                        

                    Dados do Prestador de Serviço / Detalhes da Factura
                    
                        
                        

                        Identificação do Prestador de Serviço
                        
                        
                            
                                
                                    NIF em Angola?
                                        SimNãoSimSimNão
                                        
                                    
                                            Número Fiscal*
                                                
                                                
                                            
                                            Nome/Conta*
                                                 
                                                
                                            
                                        N° Declaração de Conformidade
                                            
                                            
                                            ui-button
                                            
                                        
                                
                            
                        
                        Detalhes da Factura
                        
                        
                            
                                

                                    Sector de Prestação de Serviço Petrolífero?
                                        NãoSimNãoNãoSim
                                        
                                    

                                    Número da Factura*
                                        
                                        
                                    
                                    Descrição do Serviço*
                                        
                                        
                                    
                                    Data da Factura*
                                        
                                        
                                    
                                    Data do Pagamento*
                                        
                                        
                                    
                                    Valor da Factura
                                        
                                        
                                    
                                    Valor Pago
                                        
                                        
                                    
                                    Valor Sujeito a Retenção
                                        
                                        
                                    
                                
                            
                        
                        
                            
                                
                                    Taxa
                                        
                                        
                                    
                                    Imposto Retido
                                        
                                        
                                    
                                
                            
                        
                        
                            AdicionarSair
                            
                        
                    EXCELCSVXMLUpload de Ficheiro
                    
                        
                            Upload de Ficheiro*
                                EXCELCSVXMLEXCEL
                                
                            
                        
                        
                            Importar Mapa
                            
                        
                        
                            
                                
                                    Após seleccionar o ficheiro aguarde esta janela fechar automaticamente ao término da importação. Este processo pode demorar alguns minutos.
                                
                                
                                
                                    SeleccionarVoltar
                                    
                                
                            
                        
                    EXCELCSVXMLTemplate de Ficheiro de Dados do Prestador de Serviço
                    
                        
                            Tipo de Ficheiro*
                                EXCELCSVXMLEXCEL
                                
                            
                        
                        DownloadVoltar
                        
                    PDFEXCELGeralPor Prestadorid(&quot;dialog_modal&quot;)Prezado contribuinte, não utilizar vírgula ( , ) ou apóstrofe ( &quot; , &quot;'&quot; , &quot; ) ou ponto e vírgula ( ; )&quot;))]</value>
      <webElementGuid>0777fc9c-3a5f-412b-8f37-d683c2d3224e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
